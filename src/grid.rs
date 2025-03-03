use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_grid);
    }
}

// TheGrid is used for trakcing the grid map that ants move on and dirt is placed on
#[derive(Resource)]
pub struct TheGrid {
    pub grid: Vec<GridCell>,
    pub height: usize,
    pub width: usize,
}

// A GridCell is used to track a location and draw a debug sprite onto that location
#[derive(Bundle)]
pub struct GridCell {
    location: GridLocation,
    sprite: SpriteBundle,
}

// A GridLocation is used as cordinates for the map at large
#[derive(Component)]
pub struct GridLocation {
    pub x: usize,
    pub y: usize,
}

// Sets up a default 5x5 grid
// TODO: make this fill the whole window with a grid
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

// initializes the grid for the world
fn make_grid(mut commands: Commands) {
    commands.init_resource::<TheGrid>();
}
