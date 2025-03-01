use bevy::{math::vec3, prelude::*};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

// a marker enum that holds the type of AI to use for movement
pub enum AIType {
    AntAI,
    EggAI,
}

// a MovementAI component that contains everything needed for the movement system
#[derive(Component)]
pub struct MovementAI {
    pub ai_type: AIType,
    pub movement_info: MovementInfo,
}

// a struct that holds information about an ants movement capabilities
pub struct MovementInfo {
    pub max_speed: f32,
    pub current_speed: f32,
    pub moving_direction: Option<Direction>,
}

// an enum that holds the movement directions
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// a default implementation for MovementAI
impl Default for MovementAI {
    fn default() -> Self {
        Self {
            ai_type: AIType::AntAI,
            movement_info: MovementInfo { ..default() },
        }
    }
}

// a default implementation for MovementInfo
impl Default for MovementInfo {
    fn default() -> Self {
        Self {
            max_speed: 5.0,
            current_speed: 5.0,
            moving_direction: None,
        }
    }
}

// this allows for a Direction to be multiplied by a f32 magnitude
impl std::ops::Mul<f32> for Direction {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        vec3(rhs, rhs, 0.0)
            * match self {
                Direction::Up => vec3(0.0, 1.0, 0.0),
                Direction::Down => vec3(0.0, -1.0, 0.0),
                Direction::Left => vec3(-1.0, 0.0, 0.0),
                Direction::Right => vec3(1.0, 0.0, 0.0),
            }
    }
}

// currently this, using the element Query-ed uses the MovementAI and moves the entity Transform
// accordingly
fn movement(time: Res<Time>, mut ant_query: Query<(&MovementAI, &mut Transform)>) {
    for (ai, mut transform) in ant_query.iter_mut() {
        let MovementAI {
            ai_type,
            movement_info,
        } = ai;

        match ai_type {
            // currently these move ants in the direction from MovementInfo by their current_speed
            AIType::AntAI => {
                if let Some(direction) = movement_info.moving_direction {
                    let distance = time.delta_seconds()
                        * f32::min(movement_info.current_speed, movement_info.max_speed);
                    transform.translation += direction * distance;
                }
            }
            // TODO: separate Eggs into a separate bundle so that they don't waste cycles
            AIType::EggAI => {
                continue;
            }
        }
    }
}
