use crate::game::game_state::game_state::new_game_state;
use ui::app_view::render_app;
use ui::input::event_handling::{event, update};

mod game;
mod ui;

const ZOOM_MAX: f64 = 2.0;
const ZOOM_MIN: f64 = 0.5;

fn main() {
    nannou::app(new_game_state)
        .event(event)
        .update(update)
        .simple_window(render_app)
        .run();
}
