//! Bevy client migration: keep the original wgpu client available during parity work.
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rustcraft (Bevy)".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 24.0, 0.0).looking_at(Vec3::new(0.0, 12.0, -20.0), Vec3::Y),
    ));
}
