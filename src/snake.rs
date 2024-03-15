use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    event::{GameOverEvent, SnakeCollideEvent},
    game_state::{GameState, Score},
};
pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition { x: 0.0, y: 0.0 })
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastDirection(Vec2::ZERO))
            .insert_resource(SnakeSpeed(200.0))
            .add_systems(Startup, spawn_snake)
            .add_systems(
                Update,
                (
                    track_mouse_movements,
                    snake_head_movement,
                    add_snake_segment,
                    move_snake_segments,
                    check_snake_self_collision,
                    update_segment_collision_flag,
                ),
            );
    }
}

#[derive(Resource, Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource)]
struct MousePosition {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component, Default)]
pub struct SnakeSegment {
    ignore_collision: bool,
    collision_timer: Timer,
}

#[derive(Resource)]
pub struct SnakeSpeed(pub f32);

#[derive(Resource)]
struct LastDirection(Vec2);

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_segments: ResMut<SnakeSegments>,
) {
    let snake = commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(10.0))),
            material: materials.add(Color::GREEN),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(SnakeHead)
        .insert(SnakeSegment::default())
        .insert(Name::new("SnakeHead"))
        .id();

    snake_segments.0.push(snake);
}

fn track_mouse_movements(
    mut mouse_motion_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
    mut windows: Query<&mut Window>,
) {
    let window = windows.single_mut();
    for event in mouse_motion_events.read() {
        let position = event.position - Vec2::new(window.width() / 2.0, window.height() / 2.0);
        mouse_position.x = position.x;
        mouse_position.y = -position.y;
    }
}

fn snake_head_movement(
    snake_speed: ResMut<SnakeSpeed>,
    time: Res<Time>,
    mouse_position: Res<MousePosition>,
    mut snake_last_direction: ResMut<LastDirection>,
    mut query: Query<(&mut Transform, Entity), With<SnakeHead>>,
    state: Res<State<GameState>>,
) {
    if state.get() != &GameState::InGame {
        return;
    }
    for (mut transform, _entity) in query.iter_mut() {
        let snake_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let mouse_pos = Vec2::new(mouse_position.x, mouse_position.y);
        if (mouse_pos - snake_pos).length() > 1.0 {
            snake_last_direction.0 = (mouse_pos - snake_pos).normalize_or_zero();
        }

        let angle_to_mouse = Vec2::X.angle_between(snake_last_direction.0);

        transform.rotation = Quat::from_rotation_z(angle_to_mouse);
        transform.translation +=
            (snake_last_direction.0 * snake_speed.0 * time.delta_seconds()).extend(0.0);
    }
}

fn add_snake_segment(
    mut commands: Commands,
    mut snake_collide_event: EventReader<SnakeCollideEvent>,
    mut snake_segments: ResMut<SnakeSegments>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut snake_speed: ResMut<SnakeSpeed>,
    segment_query: Query<&Transform, With<SnakeSegment>>,
    mut score: ResMut<Score>,
    state: Res<State<GameState>>,
) {
    if state.get() != &GameState::InGame {
        return;
    }
    let snake_segment_mesh = meshes.add(Circle::new(10.0));
    let snake_segment_material = materials.add(Color::GREEN);

    for _ in snake_collide_event.read() {
        info!("Snake collided event received");

        if let Some(tail_segment) = snake_segments.0.last() {
            if let Ok(segment_transform) = segment_query.get(*tail_segment) {
                let mut transform = Transform::from_translation(segment_transform.translation);
                transform.rotation = segment_transform.rotation;

                let snake_segment = commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(snake_segment_mesh.clone()),
                        material: snake_segment_material.clone(),
                        transform,
                        ..default()
                    })
                    .insert(SnakeSegment {
                        ignore_collision: true,
                        collision_timer: Timer::from_seconds(2.0, TimerMode::Repeating), // Newly spawned segments will start with collision ignored
                    })
                    .insert(Name::new("SnakeSegment"))
                    .id();

                snake_segments.0.push(snake_segment);
                snake_speed.0 += 10.0;
                score.0 += 1;
            }
        }
    }
}

fn move_snake_segments(
    segments: Res<SnakeSegments>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    snake_speed: ResMut<SnakeSpeed>,
    state: Res<State<GameState>>,
) {
    if state.get() != &GameState::InGame {
        return;
    }
    let mut previous_position = Vec3::ZERO;
    let mut previous_rotation = Quat::IDENTITY;

    if let Ok(head_transform) = transforms.get(segments.0[0]) {
        previous_position = head_transform.translation;
        previous_rotation = head_transform.rotation;
    }
    let base_lerp_factor = 10.0 * time.delta_seconds();
    let lerp_factor = base_lerp_factor + (snake_speed.0 * 0.0003);

    for segment in segments.0.iter().skip(1) {
        if let Ok(mut transform) = transforms.get_mut(*segment) {
            let current_position = transform.translation;
            let current_rotation = transform.rotation;

            transform.translation = transform.translation.lerp(previous_position, lerp_factor);
            transform.rotation = transform.rotation.slerp(previous_rotation, lerp_factor);

            previous_position = current_position;
            previous_rotation = current_rotation;
        }
    }
}

fn check_snake_self_collision(
    _commands: Commands,
    head_query: Query<&Transform, With<SnakeHead>>,
    segment_query: Query<(Entity, &Transform, &SnakeSegment), Without<SnakeHead>>,
    mut game_over_event: EventWriter<GameOverEvent>,
    _next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(head_transform) = head_query.get_single() {
        let head_position = head_transform.translation.truncate();
        let snake_radius = 10.0;

        for (_segment_entity, segment_transform, segment) in segment_query.iter() {
            if segment.ignore_collision {
                continue;
            }

            let segment_position = segment_transform.translation.truncate();
            let distance = head_position.distance(segment_position);

            if distance + 7.0 < snake_radius {
                println!("Collision with snake segment detected! Game Over.");
                game_over_event.send(GameOverEvent);
            }
        }
    }
}

fn update_segment_collision_flag(mut segment_query: Query<&mut SnakeSegment>, time: Res<Time>) {
    for mut segment in segment_query.iter_mut() {
        if segment.ignore_collision {
            segment.collision_timer.tick(time.delta());
            if segment.collision_timer.just_finished() {
                segment.ignore_collision = false;
            }
        }
    }
}
