pub mod game_state {
    use nannou::prelude::Point2;
    use nannou_egui::Egui;

    use crate::game::ship::Ship;

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
}
