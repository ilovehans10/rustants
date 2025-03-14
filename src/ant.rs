use bevy::{math::vec3, prelude::*};

use crate::movement::{AIType, Direction, MovementAI, MovementInfo};

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ants);
    }
}

// a marker component that can be queried to find the role of the ant
#[derive(Component)]
pub enum AntRole {
    Egg,
    Grub,
    Adult,
    Queen,
}

// a bundle that contains everything needed to create an ant
// TODO: refactor this into a CreatureBundle that no longer contains a MovementAI
#[derive(Bundle)]
struct AntBundle {
    sprite: SpriteBundle,
    ant_role: AntRole,
    movement_ai: MovementAI,
}

// a default implementation for AntBundle
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

// this spawns a test variety of ants
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
        movement_ai: MovementAI {
            movement_info: MovementInfo {
                max_speed: 1.0,
                current_speed: 1.0,
                moving_direction: Some(Direction::Left),
            },
            ..default()
        },
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
        movement_ai: MovementAI {
            movement_info: MovementInfo {
                max_speed: 2.0,
                current_speed: 2.0,
                moving_direction: Some(Direction::Down),
            },
            ..default()
        },
        ..default()
    });
}
