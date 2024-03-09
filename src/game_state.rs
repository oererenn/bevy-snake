use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, game_state_system);
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
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
                info(format!("Game Paused"));
            }
            GameState::Paused => {
                next_state.set(GameState::InGame);
                info(format!("Game Resumed"));
            }
        }
        info(state.get());
    }
}
