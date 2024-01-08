mod utils;
mod cursor;
mod piping;
mod grid;

use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig
};


use cursor::{update_cursor_position, CursorWorldCoords};
use piping::handle_mouse_click;
use utils::colours::{GamePallete, get_colour};
use utils::game_settings::GameSettings;
use grid::GridPlugin;


const GAME_TITLE: &str = env!("CARGO_PKG_NAME");
const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
static SETTINGS_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/game_settings.toml"));



fn main() {

    let game_settings = GameSettings::default();

    // TODO: use bevy asset loader for load screen.
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())  // this reduces blur for pixel art
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("{}_v{}", GAME_TITLE, GAME_VERSION),
                        resolution: (game_settings.window.resolution.width as f32, game_settings.window.resolution.height as f32).into(),
                        resizable: game_settings.window.resizable,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .init_resource::<GameSettings>()
        .init_resource::<CursorWorldCoords>()
        .add_systems(Startup, setup)
        .add_plugins(GridPlugin)
        .add_systems(Update, (update_cursor_position, handle_mouse_click).chain())
        .run();
}


/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(get_colour(GamePallete::DarkSeaGreen)),
            },
            ..default()
        },
        MainCamera)
    );
}

