use bevy::prelude::Color;


pub enum AssetType {
    StraightPipe,
    Bend,
    Tee,
    Quad,
    Pump,
    PumpStation,
    Home,
    Business
}


pub fn get_asset(color: AssetType) -> Color {
    // TODO: change to return asset
    let hex = match color {
        AssetType::StraightPipe => "bbb094",
        AssetType::Bend => "807665",
        AssetType::Tee => "595246",
        AssetType::Quad => "333333",
        AssetType::Pump => "191f22",
        AssetType::PumpStation => "2f4443",
        AssetType::Home => "3b5e58",
        AssetType::Business => "5a8c6c",
    };

    Color::hex(hex).unwrap_or(Color::rgb(0.0,0.0,0.0))
}