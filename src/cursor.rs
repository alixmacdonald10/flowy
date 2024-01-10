use std::collections::{ VecDeque, HashMap };

use bevy::{
    prelude::*,
    sprite::{SpriteBundle, Sprite},
    utils::Uuid,
};

use crate::{utils::colours::{GamePallete, get_colour}, grid::grid::XYIndex};
use crate::grid::grid::{GridSettings, Grid, CursorGridIdx};
use crate::GameSettings;



pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlacingComponents>()
            .init_resource::<DeletingComponents>()
            .init_resource::<DumbScheduler>()
            .add_systems(Update, handle_mouse_click)
            .add_systems(Update, (toggle_cell_occupation, handle_equipment).chain());
    }
}

#[derive(Resource, Default)]
pub struct PlacingComponents(bool);

#[derive(Resource, Default)]
pub struct DeletingComponents(bool);

#[derive(Resource, Default)]
pub struct DumbScheduler(VecDeque<Uuid>); 

/// Start or end placement/deletion mode when the mouse is clicked.
/// Placement mode is controlled with left mouse button.
/// Deletion mode is controlled with right mouse button.
/// Only a single mode can be active at a time.
pub fn handle_mouse_click(
    input: Res<Input<MouseButton>>,
    mut placement_toggle: ResMut<PlacingComponents>,
    mut deletion_toggle: ResMut<DeletingComponents>,
) {

    if input.just_pressed(MouseButton::Left) && deletion_toggle.0 == false {

        if placement_toggle.0 == true {
            placement_toggle.0 = false;
            println!("Ending placement");
        } else {
            println!("Starting placement");
            placement_toggle.0 = true;
        }
    } else if input.just_pressed(MouseButton::Right) && placement_toggle.0 == false {

        if deletion_toggle.0 == true {
            deletion_toggle.0 = false;
            println!("Ending deletion");
        } else {
            println!("Starting deletion");
            deletion_toggle.0 = true;
        }
    }
}


/// If you are in placement mode, the cells your mouse moves over are toggled to occupied.
/// If you are in deletion mode, the cells your mouse moves over are toggled to unoccupied.
pub fn toggle_cell_occupation(
    placement_toggle: ResMut<PlacingComponents>,
    deletion_toggle: ResMut<DeletingComponents>,
    cursor_idx: Res<CursorGridIdx>,
    mut grid: ResMut<Grid>,
    mut queue: ResMut<DumbScheduler>,
) {
    if let Some(cursor_index) = cursor_idx.index {
        let current_cell = grid.cells.get_mut(&cursor_index).unwrap();  // at this point we know it exists so unwrap fine

        if placement_toggle.0 == true {
            if current_cell.occupied == false {
                current_cell.occupied = true;
                println!("Placing equipment at {:#?}", cursor_index);
                queue.0.push_front(cursor_index);
            }
        } else if deletion_toggle.0 == true {
            if current_cell.occupied == true {
                current_cell.occupied = false;
                println!("Removing equipment at {:#?}", cursor_index);
                queue.0.push_front(cursor_index);
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Equipment {
    cell_idx: Uuid
}

#[derive(Component)]
pub struct SpawnedEquipment;


pub fn handle_equipment(
    mut queue: ResMut<DumbScheduler>,
    grid_settings: Res<GridSettings>,
    grid: Res<Grid>,
    mut commands: Commands,
    existing_equipment: Query<(Entity, &Equipment), With<SpawnedEquipment>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {

    let mut existing_entities = HashMap::new();
    for entity in existing_equipment.into_iter() {
        existing_entities.insert(entity.1.cell_idx, entity);
    };

    for cell_idx in queue.0.drain(..) {
        match existing_entities.get(&cell_idx) {
            Some(entity) => {
                commands.entity(entity.0).despawn();
            },
            None => {
                
                let (camera, camera_transform) = q_camera.single();
                let current_cell = grid.cells.get(&cell_idx).unwrap();
                let cell_centre = &current_cell.centre;
            
                if let Some(ray) = camera.viewport_to_world(camera_transform, Vec2::new(cell_centre.x as f32, cell_centre.y as f32)) {
                    let truncated_ray = ray.origin.truncate();

                    commands.spawn(
                        (
                            Equipment{ cell_idx },
                            SpawnedEquipment,
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(grid_settings.cell_width as f32, grid_settings.cell_height as f32)),
                                    color: get_colour(GamePallete::Coconut),
                                    ..default()
                                },
                                transform: Transform::from_xyz(truncated_ray.x, truncated_ray.y, 0.0),
                                ..default()
                            }
                        )
                    );
                }
            }
        }
    }
}
