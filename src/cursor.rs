use bevy::prelude::*;

use crate::utils::colours::{GamePallete, get_colour};


pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_cursor, define_cursor_mode).chain())
            .add_systems(Update, handle_mouse_click)
            .add_systems(Update, render_cursor_mode_text);
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

    // TODO: there is probably a better way to do this
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

pub fn define_cursor_mode(
    mut commands: Commands
) {
    let font_handle: Handle<Font> = Default::default();

    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Cursor Mode:  ",
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: get_colour(GamePallete::Feldgrau),
            },
        ),
        TextSection::new(
            "",
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: get_colour(GamePallete::JapaneseIndigo),
            },
        )]),
        CursorModeText
    ));
}

fn render_cursor_mode_text(
    q_cursor: Query<(Entity, &Cursor, Option<&PlacingComponents>, Option<&DeletingComponents>)>,
    mut q_cursor_mode_text: Query<&mut Text, With<CursorModeText>>,
) {
    let (entity, _, _, _) = q_cursor.get_single().unwrap();
    let mode = match (q_cursor.get_component::<PlacingComponents>(entity), q_cursor.get_component::<DeletingComponents>(entity)) {
        (Ok(_), Err(_)) => "Placing",
        (Err(_), Ok(_)) => "Deleting",
        (Err(_), Err(_)) => "None",
        (Ok(_), Ok(_)) => panic!("Cursor is in both placing and deleting mode!"),
    };
    for mut text in &mut q_cursor_mode_text {
        text.sections[1].value = mode.to_string();
    }
}
