use nannou::Draw;

use crate::game::game_state::game_state::GameState;

pub trait Drawable {
    fn draw(self: &Self, draw: &Draw, model: &GameState) -> ();
}
