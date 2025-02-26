use bevy::{math::vec3, prelude::*};

use crate::movement::{AIType, Direction, MovementAI, MovementInfo};

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
    movement_ai: MovementAI,
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
            movement_ai: MovementAI { ..default() },
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
    commands.spawn(AntBundle {
        movement_ai: MovementAI {
            movement_info: MovementInfo {
                moving_direction: Some(Direction::Up),
                ..default()
            },
            ..default()
        },
        ..default()
    });
    commands.spawn(AntBundle {
        ant_role: AntRole::Grub,
        ..default()
    });
    commands.spawn(AntBundle {
        ant_role: AntRole::Egg,
        movement_ai: MovementAI {
            ai_type: AIType::EggAI,
            ..default()
        },
        ..default()
    });
    commands.spawn(AntBundle {
        ant_role: AntRole::Queen,
        ..default()
    });
}
