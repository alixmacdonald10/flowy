use std::collections::HashMap;
use std::fmt::Debug;

use bevy::{
    prelude::*,
    utils::Uuid,
    window::PrimaryWindow
};

use crate::utils::game_settings::GameSettings;

// TODO: DECOUPLE GAME SETTINGS FROM GRID PLUGIN
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GridIndex>()
            .init_resource::<GridSettings>()
            .init_resource::<XYIndex>()
            .init_resource::<Grid>()
            .init_resource::<CursorGridIdx>()
            .add_systems(Startup, (create_grid_index, create_xy_index, create_grid).chain())
            .add_systems(Update, update_cursor_idx);
    }
}

/// Used to create the uuid of each cell in the grid
#[derive(Resource, Default, Debug)]
pub struct GridIndex {
    pub index: HashMap<CellCentre, Uuid>
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct CellCentre {
    x: i32,
    y: i32,
}

impl CellCentre {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct GridSettings {
    pub cell_width: i32,
    pub cell_height: i32,
    pub grid_width: i32,
    pub grid_height: i32,
    pub number_of_horizontal_cells: i32,
    pub number_of_vertical_cells: i32,
    /// A vector of all cell centre points along the x axis
    pub x_centers_vec: Vec<i32>,
    /// A vector of all cell centre points along the y axis
    pub y_centers_vec: Vec<i32>
}


fn create_grid_index(
    mut grid_index: ResMut<GridIndex>,
    game_settings: Res<GameSettings>,
    mut grid_settings: ResMut<GridSettings>
) {
    let cell_width = game_settings.grid.cell_width;
    let cell_height = game_settings.grid.cell_height;
    let grid_width = game_settings.window.resolution.width;
    let grid_height = game_settings.window.resolution.height;

    let number_of_horizontal_cells = grid_width / cell_width;
    let number_of_vertical_cells = grid_height / cell_height;

    for horiz_cell in 0..number_of_horizontal_cells {
        for vert_cell in 0..number_of_vertical_cells {
            let left = horiz_cell * cell_width;
            let right = left + cell_width;
            let x = left + (cell_width / 2);
            let top = vert_cell * cell_height;
            let bottom = top + cell_height;
            let y = top + (cell_height / 2);

            grid_index.index.insert(
                CellCentre::new(x, y),
                Uuid::new_v4()
            );
        }
    }

    // update grid settings resource for faster compute in the next stages
    grid_settings.cell_width = cell_width;
    grid_settings.cell_height = cell_height;
    grid_settings.grid_width = grid_width;
    grid_settings.grid_height = grid_height;
    grid_settings.number_of_horizontal_cells = number_of_horizontal_cells;
    grid_settings.number_of_vertical_cells = number_of_vertical_cells;

    let x_start_centre = grid_settings.cell_width / 2;
    let x_centre_vec = (x_start_centre..grid_settings.grid_width)
        .step_by(grid_settings.cell_width as usize)
        .collect::<Vec<i32>>();
    grid_settings.x_centers_vec = x_centre_vec.to_owned();

    let y_start_centre = grid_settings.cell_height / 2;
    let y_centre_vec = (y_start_centre..grid_settings.grid_height)
        .step_by(grid_settings.cell_height as usize)
        .collect::<Vec<i32>>();
    grid_settings.y_centers_vec = y_centre_vec.to_owned();
}

/// Used as a lookup for cursor x,y position and returns uuid
#[derive(Resource, Default, Debug)]
pub struct XYIndex {
    pub index: HashMap<(i32, i32), Uuid>
}

fn create_xy_index(
    grid_index: Res<GridIndex>,
    grid_settings: Res<GridSettings>,
    mut xy_index: ResMut<XYIndex>
) {
    for x in 0..grid_settings.grid_width {
        for y in 0..grid_settings.grid_height {
            let closest_x_centre = find_closest_value(&grid_settings.x_centers_vec, &x);
            let closest_y_centre = find_closest_value(&grid_settings.y_centers_vec, &y);
            xy_index.index.insert(
                (x, y),
                grid_index.index.get(&CellCentre::new(closest_x_centre, closest_y_centre)).unwrap().to_owned()
            );
        }
    }
}

/// Used to find cell information from cursor uuid
#[derive(Resource, Default, Debug)]
pub struct Grid {
    pub cells: HashMap<Uuid, Cell>
}


#[derive(Resource, Default, Debug)]
pub struct Cell {
    pub uuid: Uuid,
    /// The x,y position of the cell in the grid
    pub centre: (i32, i32),
    /// The bounds of the cell (Left, Right, Top, Bottom)
    pub bounds: (i32, i32, i32, i32),
    /// Whether the cell is occupied by an entity
    pub occupied: bool,
    /// The uuid of the entity that is occupying this cell
    pub occupied_by: Option<Uuid>,
    /// The uuids of the neighbour cells
    pub neighbours: HashMap<String, CellNeighours>
}

fn create_grid(
    grid_index: Res<GridIndex>,
    grid_settings: Res<GridSettings>,
    mut grid: ResMut<Grid>,
) {
    for grid_cell in grid_index.index.iter() {
        let centre_x = grid_cell.0.x;
        let centre_y = grid_cell.0.y;
        let left = centre_x - (grid_settings.cell_width / 2);
        let right = centre_x + (grid_settings.cell_width / 2);
        let top = centre_y - (grid_settings.cell_height / 2);
        let bottom = centre_y + (grid_settings.cell_height / 2);

        let cell_uuid = grid_cell.1.to_owned();

        grid.cells.insert(
            cell_uuid,
            Cell {
                uuid: cell_uuid,
                centre: (centre_x, centre_y),
                bounds: (left, right, top, bottom),
                occupied: false,
                occupied_by: None,
                neighbours: HashMap::new()
            }
        );
    }
    println!("{:#?}", grid);
}

#[derive(Debug)]
pub struct CellNeighours {
    pub left: Option<Uuid>,
    pub right: Option<Uuid>,
    pub top: Option<Uuid>,
    pub bottom: Option<Uuid>
}


/// Used to find the closest value in a vector to a target value. This uses binary search for O(log n) time complexity
/// which will help with large vectors.
fn find_closest_value(vector: &Vec<i32>, target: &i32) -> i32 {
    let idx = match vector.binary_search_by(|&x| x.cmp(target)) {
        Ok(idx) => idx, // Exact match
        Err(idx) => {
            if idx > 0 && (idx == vector.len() || (vector[idx] - target).abs() > (vector[idx - 1] - target).abs()) {
                idx - 1
            } else {
                idx
            }
        }
    };

    vector[idx].to_owned()
}



/// Used to identify the position of our cursor relative to the main window camera.
#[derive(Resource, Default)]
pub struct CursorGridIdx {
    pub index: Option<Uuid>
}


pub fn update_cursor_idx(
    mut cursor_idx: ResMut<CursorGridIdx>,
    grid_settings: Res<GridSettings>,
    xy_index: Res<XYIndex>,
    q_window: Query<&Window, With<PrimaryWindow>>
) {
    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .map(|cursor| (cursor.x as i32, cursor.y as i32)) {
            let cell_centre = (
                find_closest_value(&grid_settings.x_centers_vec, &world_position.0),
                find_closest_value(&grid_settings.y_centers_vec, &world_position.1)
            );
            match xy_index.index.get(&cell_centre) {
                Some(idx) => {
                    println!("Cell centre: {:#?}", cell_centre);
                    println!("Cell index: {:#?}", idx);
                    cursor_idx.index = Some(idx.to_owned());
                },
                None => {
                    cursor_idx.index = None;
                }
            }
        }
}
