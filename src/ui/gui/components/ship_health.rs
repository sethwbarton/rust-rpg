use crate::game::game_state::game_state::GameState;
use crate::ui::gui::ClickableGuiElement;
use nannou::geom::Point2;
use nannou::{App, Draw};

pub struct ShipHealthUi {}

impl ClickableGuiElement for ShipHealthUi {
    fn on_click(self: &Self, app: &App, state: &GameState) -> () {
        todo!()
    }

    fn is_in(self: &Self, x_y: Point2) -> bool {
        todo!()
    }

    fn draw(self: &Self, app: &App, state: &GameState, draw_instance: &Draw) -> () {
        draw_instance.text("Ship Health: 100/100");
    }
}
