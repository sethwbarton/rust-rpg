use crate::game::drawable::Drawable;
use crate::game::game_state::game_state::GameState;
use nannou::Draw;

pub struct Thargoid {
    pub hp: i32,
}

impl Drawable for Thargoid {
    fn draw(self: &Self, draw: &Draw, model: &GameState) -> () {}
}

impl Thargoid {
    pub fn spawn() -> Self {
        Thargoid { hp: 100 }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::entities::ship::Ship;
    use crate::game::entities::thargoid::Thargoid;
    use crate::game::game_state::game_state::{GameState, Settings};
    use nannou::geom::pt2;

    fn new_mock_game_state() -> GameState {
        GameState {
            ship: Ship::new(),
            scale: 0.0,
            transform: pt2(0.0, 0.0),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
            egui: None,
        }
    }

    #[test]
    fn spawned_thargoids_have_100hp() {
        let new_game_state = new_mock_game_state();
        let test_thargoid = Thargoid::spawn();
        assert_eq!(test_thargoid.hp, 100);
    }
}
