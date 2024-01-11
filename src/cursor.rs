use bevy::prelude::*;



pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_cursor)
            .add_systems(Update, handle_mouse_click);
            // .add_systems(Update, render_cursor_mode_text);
    }
}

#[derive(Component, Default, Debug)]
pub struct Cursor;

#[derive(Component, Default, Debug)]
pub struct PlacingComponents;

#[derive(Component, Default, Debug)]
pub struct DeletingComponents;

/// Identifies the cursor mode text
#[derive(Component, Default)]
pub struct CursorModeText; 


fn spawn_cursor(
    mut commands: Commands
) {
    commands.spawn(Cursor);
}

/// Start or end placement/deletion mode when the mouse is clicked.
/// Placement mode is controlled with left mouse button.
/// Deletion mode is controlled with right mouse button.
/// Only a single mode can be active at a time.
pub fn handle_mouse_click(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    q_cursor: Query<(Entity, &Cursor, Option<&PlacingComponents>, Option<&DeletingComponents>)>
) {
    let (entity, _, _, _) = q_cursor.get_single().unwrap();

    if input.just_pressed(MouseButton::Left) {
        if let (Ok(_), Err(_)) = (q_cursor.get_component::<PlacingComponents>(entity), q_cursor.get_component::<DeletingComponents>(entity)) {
            commands.entity(entity)
                .remove::<PlacingComponents>();
        } 
        if let (Err(_), Err(_)) = (q_cursor.get_component::<PlacingComponents>(entity), q_cursor.get_component::<DeletingComponents>(entity)) {
            commands.entity(entity)
                .insert(PlacingComponents);
        }
    } else if input.just_pressed(MouseButton::Right) {
        commands.entity(entity).log_components();
        if let (Ok(_), Err(_)) = (q_cursor.get_component::<DeletingComponents>(entity), q_cursor.get_component::<PlacingComponents>(entity)) {
            commands.entity(entity)
                .remove::<DeletingComponents>();
        }
        if let (Err(_), Err(_)) = (q_cursor.get_component::<DeletingComponents>(entity), q_cursor.get_component::<PlacingComponents>(entity)) {
            commands.entity(entity)
                .insert(DeletingComponents);
        } 
    }
}

// pub fn define_budget(
//     mut commands: Commands,
//     mut cursor: ResMut<Budget>,
// ) {
//     let font_handle: Handle<Font> = Default::default();
//     budget.0 = game_settings.gameplay.budget;

//     commands.spawn((TextBundle::from_sections([
//         TextSection::new(
//             "Budget:  ",
//             TextStyle {
//                 font: font_handle.clone(),
//                 font_size: 60.0,
//                 color: get_colour(GamePallete::Feldgrau),
//             },
//         ),
//         TextSection::new(
//             budget.0.to_string(),
//             TextStyle {
//                 font: font_handle.clone(),
//                 font_size: 60.0,
//                 color: get_colour(GamePallete::JapaneseIndigo),
//             },
//         )]),
//         BudgetText
//     ));
// }

// fn render_cursor_mode_text() {

// }
