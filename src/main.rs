use bevy::prelude::*;

mod ant;
mod grid;
mod movement;

use ant::AntPlugin;
use grid::GridPlugin;
use movement::MovementPlugin;

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, CameraMarker));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AntPlugin, GridPlugin, MovementPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}
