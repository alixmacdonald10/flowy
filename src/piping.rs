use bevy::prelude::*;

use crate::cursor::CursorWorldCoords;


pub fn handle_mouse_click(
    input: Res<Input<MouseButton>>,
    cursor_coords: ResMut<CursorWorldCoords>,
    mut commands: Commands) {

// TODO: add noises on some mouse clicks. scale these to budgets (if over make disgruntled noises and get progressively
// more disgruntled the more over budget you are) if under budget make happy noises and get progressively happier the more
// under budget you are. if you are on budget make neutral noises. if you are very close to the budget make uneasy noises.

    if input.just_pressed(MouseButton::Left) {
        println!("Left mouse button clicked, starting placement.");

        // TODO: LOG POSITION OF CLICK IN WORLD COORDS AS NEW PIPE SEGMENT
        // TODO: CHECK IF NO ACTIVE PIPE SEGMENTS AND ADD PIPE SEGMENT ID TO MAP OF PIPE SEGMENTS WITH KEY AS LOCATION, SET ACTIVE PIPE SEGEMENT TO TRUE
        // TODO: CHECK IF THERE IS AN ACTIVE PIPE SEGEMENT IF SO THEN DEACTIVATE IT AND PLACE FINAL PIPE

        // TODO: SPAWN A RANDOM THING
        println!("Spawning a sprite at {:?}", cursor_coords.0);
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(25.0, 25.0)),
                ..default()
            },
            transform: Transform::from_translation(cursor_coords.0.extend(0.0)),
            ..default()
        });
    }
}