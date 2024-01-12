use bevy::prelude::*;

use crate::game::GameOver;
use crate::utils::colours::{GamePallete, get_colour};

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct GameTimerText;


impl Default for GameTimer {
    fn default() -> Self {
        // TODO: update depending on level
        Self {
            timer: Timer::from_seconds(30.0, TimerMode::Once)
        }
    }
}


pub fn tick_game_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut game_over_writer: ResMut<Events<GameOver>>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        game_over_writer.send(GameOver);
    }
}

pub fn handle_timer_text(
    mut commands: Commands,
    timer: Res<GameTimer>,
) {
    let font_handle: Handle<Font> = Default::default();

    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Time Remaining:  ",
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: get_colour(GamePallete::Feldgrau),
            },
        ),
        TextSection::new(
            timer.timer.duration().as_secs().to_string(),
            TextStyle {
                font: font_handle.clone(),
                font_size: 60.0,
                color: get_colour(GamePallete::JapaneseIndigo),
            },
        )]),
        GameTimerText
    ));
}

pub fn render_timer_text(
    timer: Res<GameTimer>,
    mut q_timer_text: Query<&mut Text, With<GameTimerText>>,
) {
    // TODO: if within 10seconds change to red
    for mut text in &mut q_timer_text {
        text.sections[1].value = (timer.timer.remaining().as_secs_f32() as i32).to_string()
    }
}

pub fn cleanup_timer(
    mut commands: Commands,
    q_timer: Query<(Entity, With<GameTimerText>)>,
) {
    if let Ok(timer) = q_timer.get_single() {
        commands.entity(timer.0).despawn_recursive();
    }
}