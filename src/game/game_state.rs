pub mod game_state {
    use nannou::prelude::{pt2, Point2};
    use nannou::App;
    use nannou_egui::Egui;

    use crate::game::ship::Ship;
    use crate::ui::app_view::render_app;
    use crate::ui::input::event_handling::raw_window_event;

    pub struct GameState {
        pub(crate) ship: Ship,
        pub(crate) scale: f64,
        pub(crate) transform: Point2,
        pub(crate) settings: Settings,
        pub(crate) egui: Egui,
    }

    pub struct Settings {
        pub(crate) zoom_sensitivity: f64,
        pub(crate) pan_speed: f32,
    }

    pub fn new_game_state(_app: &App) -> GameState {
        let window_id = _app
            .new_window()
            .view(render_app)
            .raw_event(raw_window_event)
            .build()
            .unwrap();
        let window = _app.window(window_id).unwrap();

        let egui = Egui::from_window(&window);

        GameState {
            egui,
            ship: Ship {},
            scale: 1.0,
            transform: pt2(0.0, 0.0),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
        }
    }
}
