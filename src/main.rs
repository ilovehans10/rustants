use bevy::prelude::*;

mod ant;
use ant::AntPlugin;

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AntPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}
