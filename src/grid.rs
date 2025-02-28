use bevy::prelude::*;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_grid);
    }
}

pub struct GridPlugin;

#[derive(Resource)]
pub struct TheGrid {
    pub grid: Vec<GridLocation>,
    pub height: usize,
    pub width: usize,
}

pub struct GridLocation {}

impl Default for TheGrid {
    fn default() -> Self {
        let (height, width) = (5, 5);
        TheGrid {
            grid: Vec::with_capacity(height * width),
            height,
            width,
        }
    }
}

fn make_grid(mut commands: Commands) {
    commands.init_resource::<TheGrid>();
}
