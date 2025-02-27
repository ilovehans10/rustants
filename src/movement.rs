use bevy::{math::vec3, prelude::*};

pub struct MovementPlugin;

#[derive(Component)]
pub enum MovementAI {
    AntAI(MovementInfo),
}

pub struct MovementInfo {
    pub max_speed: f32,
    pub current_speed: f32,
}

impl Default for MovementInfo {
    fn default() -> Self {
        Self {
            max_speed: 5.0,
            current_speed: 5.0,
            //moving_direction: None,
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(time: Res<Time>, mut ant_query: Query<(&MovementAI, &mut Transform)>) {
    for (ai, mut transform) in ant_query.iter_mut() {
        match ai {
            MovementAI::AntAI(movement_info) => {
                let distance = time.delta_seconds()
                    * f32::min(movement_info.current_speed, movement_info.max_speed);
                transform.translation += vec3(5.0 * distance, 0.0, 0.0)
            }
        }
    }
}
