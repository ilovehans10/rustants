use bevy::prelude::*;

mod ant;
use ant::AntPlugin;

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, CameraMarker));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AntPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}
