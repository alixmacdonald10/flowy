mod grid;
mod cursor;
mod equipment;
mod timer;


use bevy::prelude::*;

use grid::GridPlugin;
use cursor::CursorPlugin;
use equipment::EquipmentPlugin;
use timer::{GameTimer, tick_game_timer, handle_timer_text, render_timer_text, cleanup_timer};
use crate::utils::game_settings::GameSettings;
use crate::AppState;


#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}


#[derive(Event)]
pub struct GameOver;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            .init_resource::<GameSettings>()
            .init_resource::<GameTimer>()
            .add_event::<GameOver>()
            .add_plugins(GridPlugin)
            .add_plugins(CursorPlugin)
            .add_plugins(EquipmentPlugin)
            .add_systems(OnEnter(AppState::Game), handle_timer_text)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(Update, (tick_game_timer, render_timer_text).chain()
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)))
            .add_systems(OnExit(AppState::Game), cleanup_timer);
    }
}


fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Running => {
                println!("Pausing simulation");
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
            },
            SimulationState::Paused => {
                println!("Running simulation");
                commands.insert_resource(NextState(Some(SimulationState::Running)));
            },
        }
    }
}
