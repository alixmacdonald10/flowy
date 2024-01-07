use bevy::prelude::Color;


pub enum GamePallete {
    Khaki,
    Shadow,
    Umber,
    DarkCharcoal,
    DarkJungleGreen,
    JapaneseIndigo,
    Feldgrau,
    RussianGreen,
    DarkSeaGreen,
    DarkVanilla,
    Blond,
    TurquoiseGreen,
    DesaturatedCyan,
    HookersGreen,
    Onyx,
    Wenge,
    RoseDust,
    NewYorkPink,
    Burlywood,
    Deer,
    Coconut,
    Garnet,
    Bistro
}


pub fn get_colour(color: GamePallete) -> Color {
    let hex = match color {
        GamePallete::Khaki => "bbb094",
        GamePallete::Shadow => "807665",
        GamePallete::Umber => "595246",
        GamePallete::DarkCharcoal => "333333",
        GamePallete::DarkJungleGreen => "191f22",
        GamePallete::JapaneseIndigo => "2f4443",
        GamePallete::Feldgrau => "3b5e58",
        GamePallete::RussianGreen => "5a8c6c",
        GamePallete::DarkSeaGreen => "8bb48d",
        GamePallete::DarkVanilla => "c0d0a5",
        GamePallete::Blond => "f7efc7",
        GamePallete::TurquoiseGreen => "a1cdb0",
        GamePallete::DesaturatedCyan => "709395",
        GamePallete::HookersGreen => "4a787b",
        GamePallete::Onyx => "383140",
        GamePallete::Wenge => "734d5c",
        GamePallete::RoseDust => "a76772",
        GamePallete::NewYorkPink => "cc867d",
        GamePallete::Burlywood => "e0ba8b",
        GamePallete::Deer => "c38252",
        GamePallete::Coconut => "a1563c",
        GamePallete::Garnet => "6f342d",
        GamePallete::Bistro => "44271f",
    };

    Color::hex(hex).unwrap_or(Color::rgb(0.0,0.0,0.0))
}