pub mod game_state {
    use nannou::App;
    use nannou::prelude::{Point2, pt2};

    use crate::game::ship::Ship;

    pub struct GameState {
        pub(crate) ship: Ship,
        pub(crate) scale: f64,
        pub(crate) transform: Point2,
        pub(crate) settings: Settings,
    }

    pub struct Settings {
        pub(crate) zoom_sensitivity: f64,
        pub(crate) pan_speed: f32,
    }

    pub fn new_game_state(_app: &App) -> GameState {
        GameState {
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
