use crate::game::game_state::game_state::new_game_state;
use crate::ui::event_handling::event_handling::{event, update};
use crate::ui::view::view::app_view;

mod game;
mod ui;

const ZOOM_MAX: f64 = 2.0;
const ZOOM_MIN: f64 = 0.5;

fn main() {
    nannou::app(new_game_state)
        .event(event)
        .update(update)
        .simple_window(app_view)
        .run();
}
