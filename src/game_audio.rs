use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::event::{CoinCollectedEvent, GameOverEvent};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameOverAudioPlayed(false))
            .add_systems(Update, (play_coin_audio, play_game_over_audio));
    }
}

#[derive(Component)]
pub struct CoinColledtedAudio;

#[derive(Component)]
pub struct GameOverAudio;

#[derive(Resource, Default)]
pub struct GameOverAudioPlayed(pub bool);

fn play_coin_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut coin_collected_event: EventReader<CoinCollectedEvent>,
) {
    for _ in coin_collected_event.read() {
        commands
            .spawn(AudioBundle {
                source: asset_server.load("audio/coin.ogg"),
                ..default()
            })
            .insert(CoinColledtedAudio);
    }
}

fn play_game_over_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_over_event: EventReader<GameOverEvent>,
    mut audio_played: ResMut<GameOverAudioPlayed>,
) {
    if !audio_played.0 {
        for _ in game_over_event.read() {
            commands
                .spawn(AudioBundle {
                    source: asset_server.load("audio/gameover.ogg"),
                    ..default()
                })
                .insert(GameOverAudio);
            audio_played.0 = true;
            break;
        }
    }
}
