pub mod view {
    use nannou::color::BLACK;
    use nannou::geom::Vec3;
    use nannou::prelude::Point2;
    use nannou::{App, Frame};
    use std::any::Any;
    use std::fmt::Pointer;

    use crate::game::drawable::Drawable;
    use crate::game::game_state::game_state::GameState;

    pub trait RenderedView {
        // A rendered view automatically renders all drawable items in the model
        fn render(self: &Self, _app: &App, _model: &GameState, frame: Frame) -> ();
        // To render a new gui element, it has to be registered with the view
        fn register_gui_element(self: &mut Self, element: Box<dyn ClickableGuiElement>) -> ();
    }

    pub struct View {
        gui_elements: Vec<Box<dyn ClickableGuiElement>>,
    }

    impl RenderedView for View {
        fn render(self: &Self, _app: &App, _model: &GameState, frame: Frame) -> () {
            frame.clear(BLACK);
            draw_scene(_app, _model, &frame);
            draw_ui(_app, _model, &frame, &self.gui_elements);
        }

        fn register_gui_element(&mut self, element: Box<dyn ClickableGuiElement>) -> () {
            self.gui_elements.push(element)
        }
    }

    pub fn app_view(_app: &App, _model: &GameState, frame: Frame) {
        let view = View {
            gui_elements: Vec::from([]),
        };
        view.render(_app, _model, frame);
    }

    pub trait ClickableGuiElement {
        fn on_click(self: &Self, app: &App, state: &GameState) -> ();
        // Check if x_y is inside the gui element
        fn is_in(self: &Self, x_y: Point2) -> bool;
        fn draw(self: &Self, app: &App, state: &GameState) -> ();
    }

    #[derive(Clone)]
    pub struct GuiElement {
        x: Point2,
        y: Point2,
    }

    fn draw_ui(
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

    pub fn handle_ui_clicks(_app: &App) {}

    fn draw_scene(_app: &App, _model: &GameState, frame: &Frame) {
        let scene_draw_instance = _app.draw().scale(_model.scale as f32).translate(Vec3::new(
            _model.transform.x,
            _model.transform.y,
            0.0,
        ));
        _model.ship.draw(&scene_draw_instance, _model);
        scene_draw_instance.to_frame(_app, &frame).unwrap();
    }
}
