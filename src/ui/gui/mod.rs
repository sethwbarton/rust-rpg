pub mod components;

use crate::game::game_state::game_state::GameState;
use crate::ui::gui::components::ship_health::ShipHealthUi;
use nannou::geom::Point2;
use nannou::{App, Draw, Frame};

pub trait ClickableGuiElement {
    fn on_click(self: &Self, app: &App, state: &GameState) -> ();
    fn is_in(self: &Self, x_y: Point2) -> bool;
    fn draw(self: &Self, app: &App, state: &GameState, draw_instance: &Draw) -> ();
}

pub fn draw_ui(_app: &App, state: &GameState, frame: &Frame) {
    let ui_draw_instance = _app.draw().scale(1.0);

    let ship_health = ShipHealthUi {};
    ship_health.draw(_app, state, &ui_draw_instance);

    ui_draw_instance.to_frame(_app, &frame).unwrap();
    state.egui.draw_to_frame(&frame).unwrap();
}
