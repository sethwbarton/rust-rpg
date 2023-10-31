use crate::game::game_state::GameState;
use crate::lib::{SCREEN_HEIGHT, SCREEN_WIDTH};
use console_engine::{ConsoleEngine, KeyCode};
use std::process::exit;

pub fn handle_input(game_state: &mut GameState, engine: &ConsoleEngine) {
    if engine.is_key_pressed(KeyCode::Char('w')) && game_state.player.y - 1 > 0 {
        game_state.player.y -= 1;
    }
    if engine.is_key_pressed(KeyCode::Char('s'))
        && game_state.player.y + 1 < (SCREEN_HEIGHT as i32) - 1
    {
        game_state.player.y += 1;
    }
    if engine.is_key_pressed(KeyCode::Char('a')) && game_state.player.x - 1 > 0 {
        game_state.player.x -= 1;
    }
    if engine.is_key_pressed(KeyCode::Char('d'))
        && game_state.player.x + 1 < (SCREEN_WIDTH as i32) - 1
    {
        game_state.player.x += 1;
    }
}
