use bevy::prelude::*;

mod ant;
mod movement;

use ant::AntPlugin;
use movement::MovementPlugin;

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, CameraMarker));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AntPlugin, MovementPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}
