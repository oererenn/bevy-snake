use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::prelude::*;

use crate::game_state::GameState;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CoinTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, spawn_coin);
    }
}

#[derive(Resource)]
struct CoinTimer(Timer);

#[derive(Component)]
pub struct Coin;

fn spawn_coin(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut coin_timer: ResMut<CoinTimer>,
    time: Res<Time>,
    state: Res<State<GameState>>,
) {
    if state.get() != &GameState::InGame {
        return;
    }
    if coin_timer.0.tick(time.delta()).finished() {
        let window = windows.single_mut();
        let window_size = Vec2::new(window.width(), window.height());
        let offset = 30.0;

        let mut rng = thread_rng();

        let x = rng.gen_range((-window_size.x / 2.0) + offset..(window_size.x / 2.0) - offset);
        let y = rng.gen_range((-window_size.y / 2.0) + offset..(window_size.y / 2.0) - offset);

        commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
                material: materials.add(Color::RED),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(Coin);
    }
}
