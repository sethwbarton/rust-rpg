use crate::game::game_state::game_state::GameState;
use nannou::color::BLACK;
use nannou::{App, Frame};

use crate::ui::gui::components::ship_health::ShipHealthUi;
use crate::ui::gui::ClickableGuiElement;
use crate::ui::{graphics, gui};

pub fn render_app(_app: &App, _model: &GameState, frame: Frame) {
    let mut view = View {
        gui_elements: Vec::from([]),
    };
    view.render(_app, _model, frame);
}

pub struct View {
    pub(crate) gui_elements: Vec<Box<dyn ClickableGuiElement>>,
}

pub trait RenderableView {
    // A rendered view automatically renders all drawable items in the model
    fn render(self: &Self, _app: &App, _model: &GameState, frame: Frame) -> ();
    // To render a new gui element, it has to be registered with the view
    fn register_gui_element(self: &mut Self, element: Box<dyn ClickableGuiElement>) -> ();
}

impl RenderableView for View {
    fn render(self: &Self, _app: &App, _model: &GameState, frame: Frame) -> () {
        frame.clear(BLACK);
        graphics::draw_scene(_app, _model, &frame, &self);
        gui::draw_ui(_app, _model, &frame, &self.gui_elements, &self);
    }

    fn register_gui_element(&mut self, element: Box<dyn ClickableGuiElement>) -> () {
        self.gui_elements.push(element)
    }
}
