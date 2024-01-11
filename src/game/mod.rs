mod grid;
mod cursor;
mod equipment;
mod timer;

use bevy::prelude::*;

use grid::GridPlugin;
use cursor::CursorPlugin;
use equipment::EquipmentPlugin;
use timer::{GameTimer, tick_game_timer, handle_timer_text, render_timer_text};
use crate::utils::game_settings::GameSettings;


#[derive(Event)]
pub struct GameOver;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameSettings>()
            .init_resource::<GameTimer>()
            .add_event::<GameOver>()
            .add_plugins(GridPlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(EquipmentPlugin)
            .add_systems(Startup, handle_timer_text)
            .add_systems(Update, (tick_game_timer, render_timer_text).chain());
    }
}
