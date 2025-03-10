use bevy::{math::vec3, prelude::*};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_grid);
    }
}

// TheGrid is used for trakcing the grid map that ants move on and dirt is placed on
#[derive(Component)]
pub struct GridInfo {
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
#[derive(Component, Debug)]
pub struct GridLocation {
    pub x: usize,
    pub y: usize,
}

impl std::cmp::PartialEq for GridLocation {
    fn eq(&self, lhs: &GridLocation) -> bool {
        let GridLocation { x: lhs_x, y: lhs_y } = lhs;
        let GridLocation { x: rhs_x, y: rhs_y } = self;
        lhs_x == rhs_x && lhs_y == rhs_y
    }
}

fn get_grid_cords_from_index(height: usize, width: usize, index: usize) -> Option<GridLocation> {
    if height * width <= index {
        return None;
    }
    Some(GridLocation {
        x: index % height,
        y: index / width,
    })
}

fn get_world_cords_from_index(height: usize, width: usize, index: usize) -> Option<Vec3> {
    Some(vec3(30.0, 0.0, 0.0))
}

// Sets up a default 5 by 5 grid
// TODO: make this fill the whole window with a grid
impl Default for GridInfo {
    fn default() -> Self {
        GridInfo {
            height: 5,
            width: 5,
        }
    }
}

// Generates all the grid cells for the grid window
fn generate_the_grid(height: usize, width: usize) -> Vec<GridCell> {
    let mut grid = Vec::with_capacity(height * width);
    for index in 0..=height * width - 1 {
        grid.push(GridCell {
            location: get_grid_cords_from_index(height, width, index).unwrap(),
            sprite: SpriteBundle {
                transform: Transform {
                    scale: vec3(10.0, 10.0, 0.0),
                    //translation: get_world_cords_from_index(height, width, index).unwrap(),
                    ..default()
                },
                ..default()
            },
        })
    }
    grid
}

// initializes the grid for the world
fn make_grid(mut commands: Commands) {
    commands.spawn(GridInfo { ..default() });
    commands.spawn_batch(generate_the_grid(5, 5));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_get_cords_from_index() {
        assert_eq!(
            get_grid_cords_from_index(5, 5, 0),
            Some(GridLocation { x: 0, y: 0 })
        );
        assert_eq!(
            get_grid_cords_from_index(5, 5, 1),
            Some(GridLocation { x: 1, y: 0 })
        );
        assert_eq!(
            get_grid_cords_from_index(5, 5, 5),
            Some(GridLocation { x: 0, y: 1 })
        );
        assert_eq!(
            get_grid_cords_from_index(5, 5, 6),
            Some(GridLocation { x: 1, y: 1 })
        );
        assert_eq!(get_grid_cords_from_index(5, 5, 25), None);
    }
}
