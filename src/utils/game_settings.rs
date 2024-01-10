use bevy::prelude::*;
use serde::Deserialize;
use toml;

use crate::SETTINGS_STR;


#[derive(Resource, Deserialize, Debug)]
pub struct GameSettings {
    pub window: WindowGameSettings,
    pub grid: GridGameSettings,
    pub gameplay: GameplaySettings,
    pub assets: AssetSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        // im ok with tanking the game here if theres no settings file...
        let game_settings: GameSettings = toml::from_str(SETTINGS_STR).unwrap();
        println!("{:#?}", game_settings);
        game_settings
    }
}

#[derive(Deserialize, Debug)]
pub struct WindowGameSettings {
    pub resolution: ResolutionGameSettings,
    pub resizable: bool,
}

#[derive(Deserialize, Debug)]
pub struct ResolutionGameSettings {
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, Debug)]
pub struct GridGameSettings {
    pub cell_width: i32,
    pub cell_height: i32,
}

#[derive(Deserialize, Debug)]
pub struct GameplaySettings {
    pub budget: i32,
}

#[derive(Deserialize, Debug)]
pub struct AssetSettings {
    pub straight_pipe: AssetMeta,
    pub bend: AssetMeta,
    pub tee: AssetMeta,
    pub quad: AssetMeta,
    pub pump: AssetMeta,
    pub pump_station: AssetMeta,
    pub home: AssetMeta,
    pub business: AssetMeta,
}

#[derive(Deserialize, Debug)]
pub struct AssetMeta {
    pub name: String,
    pub path: String,
    pub cost: i32,
}
