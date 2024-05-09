pub mod game_state {
    use crate::game::entities::ship::Ship;
    use crate::game::entities::thargoid::Thargoid;
    use nannou::prelude::Point2;
    use nannou_egui::Egui;

    pub struct GameState {
        pub(crate) ship: Ship,
        pub(crate) thargoids: Vec<Thargoid>,
        pub(crate) scale: f64,
        pub(crate) transform: Point2,
        pub(crate) settings: Settings,
        pub(crate) egui: Option<Egui>,
    }

    pub struct Settings {
        pub(crate) zoom_sensitivity: f64,
        pub(crate) pan_speed: f32,
    }
}
