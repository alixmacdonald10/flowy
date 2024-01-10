use bevy::prelude::*;



pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlacingComponents>()
            .init_resource::<DeletingComponents>()
            .add_systems(Update, handle_mouse_click);
    }
}

#[derive(Resource, Default)]
pub struct PlacingComponents(pub bool);

#[derive(Resource, Default)]
pub struct DeletingComponents(pub bool);


/// Start or end placement/deletion mode when the mouse is clicked.
/// Placement mode is controlled with left mouse button.
/// Deletion mode is controlled with right mouse button.
/// Only a single mode can be active at a time.
pub fn handle_mouse_click(
    input: Res<Input<MouseButton>>,
    mut placement_toggle: ResMut<PlacingComponents>,
    mut deletion_toggle: ResMut<DeletingComponents>,
) {

    if input.just_pressed(MouseButton::Left) && !deletion_toggle.0 {

        if placement_toggle.0 {
            placement_toggle.0 = false;
        } else {
            placement_toggle.0 = true;
        }
    } else if input.just_pressed(MouseButton::Right) && !placement_toggle.0 {

        if deletion_toggle.0 {
            deletion_toggle.0 = false;
        } else {
            deletion_toggle.0 = true;
        }
    }
}
