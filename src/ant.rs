use bevy::prelude::*;

#[derive(Component)]
pub struct AntPlugin;

#[derive(Component)]
pub struct Ant;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ants);
    }
}

fn spawn_ants(mut commands: Commands) {
    info!("AntPlugin loaded!");
    commands.spawn((
        SpriteBundle {..default()},
        Ant));
}
