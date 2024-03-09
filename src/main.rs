use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Awesome Snake Game".into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup_camera2d)
        .run();
}

fn setup_camera2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
