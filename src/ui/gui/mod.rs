use crate::game::game_state::game_state::GameState;
use nannou::geom::Point2;
use nannou::{App, Frame};

#[derive(Clone)]
pub struct GuiElement {
    x: Point2,
    y: Point2,
}

pub trait ClickableGuiElement {
    fn on_click(self: &Self, app: &App, state: &GameState) -> ();
    // Check if x_y is inside the gui element
    fn is_in(self: &Self, x_y: Point2) -> bool;
    fn draw(self: &Self, app: &App, state: &GameState) -> ();
}

pub fn draw_ui(
    _app: &App,
    state: &GameState,
    frame: &Frame,
    gui_elements: &Vec<Box<dyn ClickableGuiElement>>,
) {
    let ui_draw_instance = _app.draw().scale(2.0);
    ui_draw_instance.text("hello seth");
    ui_draw_instance.to_frame(_app, &frame).unwrap();

    for element in gui_elements {
        element.draw(_app, state);
    }
}
