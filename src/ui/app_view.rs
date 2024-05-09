use crate::game::game_state::game_state::GameState;
use nannou::color::BLACK;
use nannou::{App, Frame};

use crate::ui::gui::ClickableGuiElement;
use crate::ui::{app_view, graphics, gui};

pub fn render_app(_app: &App, _model: &GameState, frame: Frame) {
    frame.clear(BLACK);
    graphics::draw_scene(_app, _model, &frame);
    gui::draw_ui(_app, _model, &frame);
}
