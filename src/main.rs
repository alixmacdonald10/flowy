mod main_menu;
mod game;
mod game_over;
mod utils;

use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    app::AppExit,
};

use main_menu::MainMenuPlugin;
use utils::colours::{GamePallete, get_colour};
use utils::game_settings::GameSettings;
use game::GamePlugin;
use game_over::GameOverPlugin;

const GAME_TITLE: &str = env!("CARGO_PKG_NAME");
const GAME_VERSION: &str = env!("CARGO_PKG_VERSION");
static SETTINGS_STR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/game_settings.toml"));


#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}


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
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(GameOverPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, exit_game)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(get_colour(GamePallete::DarkSeaGreen)),
            },
            ..default()
        }
    );
}


fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut exit_writer: ResMut<Events<AppExit>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_writer.send(AppExit);
    }
}
