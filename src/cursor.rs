use bevy::{
    prelude::*,
    window::PrimaryWindow
};

use crate::MainCamera;


/// Used to identify the position of our cursor relative to the main window camera.
#[derive(Resource, Default)]
pub struct CursorWorldCoords(pub Vec2);


pub fn update_cursor_position(
    mut cursor_coords: ResMut<CursorWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        cursor_coords.0 = world_position;
        // println!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
