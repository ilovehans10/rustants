use bevy::{math::vec3, prelude::*};

pub struct MovementPlugin;

#[derive(Component)]
pub enum MovementAI {
    AntAI,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
    }
}

fn movement(time: Res<Time>, mut ant_query: Query<(&MovementAI, &mut Transform)>) {
    for (ai, mut transform) in ant_query.iter_mut() {
        let distance = time.delta_seconds();
        match ai {
            MovementAI::AntAI => transform.translation += Vec3::new(5.0 * distance, 0.0, 0.0),
        }
    }
}
