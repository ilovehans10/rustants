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
        app.add_systems(Startup, spawn_ants)
            .add_systems(Update, ant_movement);
    }
}

fn spawn_ants(mut commands: Commands) {
    info!("AntPlugin loaded!");
    commands.spawn(AntBundle { ..default() });
    commands.spawn(AntBundle {
        ant_role: AntRole::Grub,
        ..default()
    });
    commands.spawn(AntBundle {
        ant_role: AntRole::Egg,
        ..default()
    });
    commands.spawn(AntBundle {
        ant_role: AntRole::Queen,
        ..default()
    });
}

fn ant_movement(time: Res<Time>, mut ant_query: Query<(&AntRole, &mut Transform)>) {
    for (role, mut transform) in ant_query.iter_mut() {
        let distance = time.delta_seconds();
        match role {
            AntRole::Adult => transform.translation += Vec3::new(5.0 * distance, 0.0, 0.0),
            AntRole::Grub => transform.translation += Vec3::new(0.5 * distance, 0.0, 0.0),
            AntRole::Queen => transform.translation += Vec3::new(0.0, 1.0 * distance, 0.0),
            AntRole::Egg => (),
        }
    }
}
