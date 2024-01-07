use bevy::prelude::*;
use serde::Deserialize;


#[derive(Resource, Deserialize, Debug)]
pub struct GameSettings {
    pub window: WindowGameSettings,
}

#[derive(Deserialize, Debug)]
pub struct WindowGameSettings {
    pub resolution: ResolutionGameSettings,
    pub resizable: bool,
}

#[derive(Deserialize, Debug)]
pub struct ResolutionGameSettings {
    pub width: f32,
    pub height: f32,
}