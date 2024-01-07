mod utils;
mod cursor;
mod piping;

use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    window::PrimaryWindow
};
use toml;

use cursor::{update_cursor_position, CursorWorldCoords};
use piping::handle_mouse_click;
use utils::colours::{GamePallete, get_colour};
use utils::game_settings::GameSettings;


const GAME_TITLE: &str = env!("CARGO_PKG_NAME");
const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
static SETTINGS_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/game_settings.toml"));



fn main() {

    // im ok with tanking the game here if theres no settings file...
    let game_settings: GameSettings = toml::from_str(SETTINGS_STR).unwrap();
    println!("{:#?}", game_settings);

    // TODO: use bevy asset loader for load screen.
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())  // this reduces blur for pixel art
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!("{}_v{}", GAME_TITLE, GAME_VERSION),
                        resolution: (game_settings.window.resolution.width, game_settings.window.resolution.height).into(),
                        resizable: game_settings.window.resizable,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .init_resource::<CursorWorldCoords>()
        .add_systems(Startup, setup)
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

