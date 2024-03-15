mod coin;
mod event;
mod fps;
mod game_audio;
mod game_state;
mod snake;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use coin::CoinPlugin;
use event::EventPlugin;
use fps::FpsPlugin;
use game_audio::GameAudioPlugin;
use game_state::GameStatePlugin;
use snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Awesome Snake Game".into(),
                resizable: true,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins((
            SnakePlugin,
            GameStatePlugin,
            EventPlugin,
            FpsPlugin,
            CoinPlugin,
            GameAudioPlugin,
        ))
        .add_systems(Startup, setup_camera2d)
        .run();
}

fn setup_camera2d(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
