use bevy::{math::vec3, prelude::*};

#[derive(Component)]
pub struct AntPlugin;

#[derive(Component)]
pub enum AntRole {
    Egg,
    Grub,
    Adult,
    Queen,
}

#[derive(Bundle)]
struct AntBundle {
    sprite: SpriteBundle,
    ant_role: AntRole,
}

impl Default for AntBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform {
                    scale: vec3(10.0, 10.0, 0.0),
                    ..default()
                },
                ..default()
            },
            ant_role: AntRole::Adult,
        }
    }
}

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ants);
    }
}

fn spawn_ants(mut commands: Commands) {
    info!("AntPlugin loaded!");
    commands.spawn(AntBundle { ..default() });
}
