use bevy::{
    prelude::*,
    sprite::{SpriteBundle, Sprite},
    utils::Uuid,
};

use crate::utils::{
    colours::{GamePallete, get_colour},
    game_settings::GameSettings,
};
use crate::game::grid::{GridSettings, Grid, CursorGridIdx};
use crate::game::cursor::{Cursor, DeletingComponents, PlacingComponents};
use crate::AppState;
use crate::game::SimulationState;



pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Budget>()
            .add_systems(OnEnter(AppState::Game), define_budget)
            .add_systems(Update, (flag_equipment, spawn_equipment, update_budget, render_budget, despawn_equipment)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
                .chain())
            .add_systems(OnExit(AppState::Game), (cleanup_equipment, cleanup_budget));
    }
}


#[derive(Component, Debug)]
pub struct Equipment {
    pub cell_idx: Uuid,
    pub cost: Option<i32>
}

#[derive(Component)]
pub struct SpawnedEquipment;

#[derive(Component)]
pub struct ToBeSpawned;

#[derive(Component)]
pub struct ToBeDespawned;


#[derive(Resource, Default)]
pub struct Budget(i32); 

/// Identifies the budget text
#[derive(Component, Default)]
pub struct BudgetText; 

/// Defines the equipment which has already been accounted for in the budget
#[derive(Component, Default)]
pub struct BudgetedEquipment;


/// If you are in placement mode, the cells your mouse moves over are toggled to occupied and entity is flagged to be spawned.
/// If you are in deletion mode, the cells your mouse moves over are toggled to unoccupied and entity is flagged to be despawned.
pub fn flag_equipment(
    mut commands: Commands,
    cursor_idx: Res<CursorGridIdx>,
    mut grid: ResMut<Grid>,
    q_cursor: Query<(Entity, &Cursor, Option<&PlacingComponents>, Option<&DeletingComponents>)>,
    q_existing_equipment: Query<(Entity, &Equipment), With<SpawnedEquipment>>,
) {
    if let Some(cursor_index) = cursor_idx.index {
        let current_cell = grid.cells.get_mut(&cursor_index).unwrap();  // at this point we know it exists so unwrap fine

        let (cursor_entity, _, _, _) = q_cursor.single();
        if let Ok(_) = q_cursor.get_component::<PlacingComponents>(cursor_entity) {
            if current_cell.occupied == false {
                current_cell.occupied = true;
                commands.spawn(
                    (
                        Equipment{ 
                            cell_idx: current_cell.id,
                            cost: None
                        },
                        ToBeSpawned,
                    )
                );
            }
        } 
        if let Ok(_) = q_cursor.get_component::<DeletingComponents>(cursor_entity) {
            if current_cell.occupied == true {
                current_cell.occupied = false;

                for (entity, equipment) in q_existing_equipment.iter() {
                    if equipment.cell_idx == current_cell.id {
                        commands.entity(entity)
                            .insert(ToBeDespawned)
                            .remove::<SpawnedEquipment>();
                    }
                }
            }
        }     
    }
}

// TODO: create spawn_equipment and flag as spawnedEquipment. add cost to equipment! remove ToBeSpawned component
pub fn spawn_equipment(
    mut commands: Commands,
    grid: Res<Grid>,
    grid_settings: Res<GridSettings>,
    mut q_equipment_to_spawn: Query<(Entity, &mut Equipment), With<ToBeSpawned>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    for (entity, mut equipment) in q_equipment_to_spawn.iter_mut() {
        // FIXME: set equipment cost
        equipment.cost = Some(10);

        let cell_idx = equipment.cell_idx;

        let (camera, camera_transform) = q_camera.single();
        let current_cell = grid.cells.get(&cell_idx).unwrap();
        let cell_centre = &current_cell.centre;
            
        if let Some(ray) = camera.viewport_to_world(camera_transform, Vec2::new(cell_centre.x as f32, cell_centre.y as f32)) {
            let truncated_ray = ray.origin.truncate();

            // TODO: find actual type
            commands.entity(entity)
                .remove::<ToBeSpawned>()
                .insert(SpawnedEquipment)
                .insert(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(grid_settings.cell_width as f32, grid_settings.cell_height as f32)),
                        color: get_colour(GamePallete::Coconut),
                        ..default()
                    },
                    transform: Transform::from_xyz(truncated_ray.x, truncated_ray.y, 0.0),
                    ..default()
                });
            }
    }
}

pub fn despawn_equipment(
    mut commands: Commands,
    q_equipment_to_despawn: Query<(Entity, &Equipment), With<ToBeDespawned>>,
) {
    for (entity, _) in q_equipment_to_despawn.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn define_budget(
    mut commands: Commands,
    game_settings: Res<GameSettings>, 
    mut budget: ResMut<Budget>,
) {
    let font_handle: Handle<Font> = Default::default();
    budget.0 = game_settings.gameplay.budget;

    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Budget:  ",
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: get_colour(GamePallete::Feldgrau),
            },
        ),
        TextSection::new(
            budget.0.to_string(),
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: get_colour(GamePallete::JapaneseIndigo),
            },
        )]),
        BudgetText
    ));
}


pub fn update_budget(
    mut commands: Commands,
    mut budget: ResMut<Budget>,
    q_new_equipment: Query<(Entity, &Equipment), (With<SpawnedEquipment>, Without<BudgetedEquipment>)>,
    q_equipment_to_be_deleted: Query<(Entity, &Equipment), (With<BudgetedEquipment>, With<ToBeDespawned>)>,
) {
    for (entity, equipment) in q_new_equipment.iter() {
        if let Some(cost) = equipment.cost {
            budget.0 -= cost;
        }
        commands.entity(entity).insert(BudgetedEquipment);
    }
    for (entity, equipment) in q_equipment_to_be_deleted.iter() {
        if let Some(cost) = equipment.cost {
            budget.0 += cost;
        }
        commands.entity(entity).remove::<BudgetedEquipment>();
    }
}


pub fn render_budget(
    budget: ResMut<Budget>,
    mut query: Query<&mut Text, With<BudgetText>>,
) {
    for mut text in &mut query {
        text.sections[1].value = budget.0.to_string();
    }
}


fn cleanup_equipment(
    mut commands: Commands,
    q_equipment: Query<Entity, With<Equipment>>,
) {
    for entity in q_equipment.iter() {
        commands.entity(entity).despawn();
    }
}

fn cleanup_budget(
    mut commands: Commands,
    q_budget_text: Query<Entity, With<BudgetText>>,
) {
    if let Ok(entity) = q_budget_text.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}  
