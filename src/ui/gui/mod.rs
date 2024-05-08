pub mod components;

use crate::game::game_state::game_state::GameState;
use crate::ui::app_view::{RenderableView, View};
use crate::ui::gui::components::ship_health::ShipHealthUi;
use nannou::geom::Point2;
use nannou::{App, Draw, Frame};

pub trait ClickableGuiElement {
    fn on_click(self: &Self, app: &App, state: &GameState) -> ();
    // Check if x_y is inside the gui element
    fn is_in(self: &Self, x_y: Point2) -> bool;
    fn draw(self: &Self, app: &App, state: &GameState, draw_instance: &Draw) -> ();
}

pub fn draw_ui(
    _app: &App,
    state: &GameState,
    frame: &Frame,
    gui_elements: &Vec<Box<dyn ClickableGuiElement>>,
    view: &View,
) {
    // One thing I'm learning about Rust.... it might be better to write more straightforward code.
    // No dynamic dispatch.
    view.register_gui_element(ShipHealthUi {});

    for element in gui_elements {
        let ui_draw_instance = _app.draw().scale(2.0);
        element.draw(_app, state, &ui_draw_instance);
        ui_draw_instance.to_frame(_app, &frame).unwrap();
    }
}
