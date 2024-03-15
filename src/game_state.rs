use bevy::prelude::*;

use crate::{
    coin::Coin,
    event::GameOverEvent,
    game_audio::{CoinColledtedAudio, GameOverAudio, GameOverAudioPlayed},
    snake::{SnakeHead, SnakeSegment, SnakeSegments, SnakeSpeed},
};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(Score(0))
            .add_systems(Startup, (setup_reset_button, setup_score_label))
            .add_systems(
                Update,
                (
                    game_state_system,
                    reset_game,
                    score_update_system,
                    click_reset_button,
                ),
            );
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct ResetButton;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

fn game_state_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::InGame);
            }
            GameState::GameOver => {}
        }
    }
}

fn reset_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_over_event: EventReader<GameOverEvent>,
    coin_query: Query<Entity, With<Coin>>,
    segment_query: Query<(Entity, &Transform, &SnakeSegment), Without<SnakeHead>>,
    mut score: ResMut<Score>,
    mut reset_button_query: Query<&mut Visibility, With<ResetButton>>,
    game_over_audio_query: Query<Entity, With<GameOverAudio>>,
    coin_collected_query: Query<Entity, With<CoinColledtedAudio>>,
) {
    for _ in game_over_event.read() {
        next_state.set(GameState::GameOver);
        for entity in coin_query.iter() {
            commands.entity(entity).despawn();
        }

        for (entity, _, _) in segment_query.iter() {
            commands.entity(entity).despawn();
        }

        for entity in game_over_audio_query.iter() {
            commands.entity(entity).despawn();
        }

        for entity in coin_collected_query.iter() {
            commands.entity(entity).despawn();
        }

        let mut reset_button = reset_button_query.single_mut();
        *reset_button = Visibility::Visible;
        score.0 = 0;
    }
}

fn score_update_system(mut query: Query<&mut Text, With<ScoreText>>, score: ResMut<Score>) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", score.0);
    }
}

fn click_reset_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut reset_button_query: Query<&mut Visibility, With<ResetButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut snake_segments: ResMut<SnakeSegments>,
    mut snake_speed: ResMut<SnakeSpeed>,
    mut audio_played: ResMut<GameOverAudioPlayed>,
) {
    for (interaction, _color, _border_color, _children) in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            info!("Button pressed");
            let mut reset_button = reset_button_query.single_mut();
            *reset_button = Visibility::Hidden;
            next_state.set(GameState::InGame);
            snake_segments.0.truncate(1);
            snake_speed.0 = 200.0;
            audio_played.0 = false;
        }
    }
}

fn setup_reset_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            ResetButton,
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::WHITE),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Again",
                        TextStyle {
                            font: asset_server.load("font/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn setup_score_label(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("font/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            left: Val::Px(120.0),
            ..Default::default()
        }),
        ScoreText,
    ));
}
