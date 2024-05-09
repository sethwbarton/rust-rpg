use crate::game::drawable::Drawable;
use crate::game::game_state::game_state::GameState;
use nannou::geom::{pt2, Point2};
use nannou::Draw;

pub struct Thargoid {
    pub hp: i32,
    pub xy: Point2,
}

impl Drawable for Thargoid {
    fn draw(self: &Self, draw: &Draw, model: &GameState) {
        draw.xy(self.xy).ellipse().resolution(6.0);
    }
}

impl Thargoid {
    pub fn spawn(state: &GameState) -> Self {
        Thargoid {
            hp: 100,
            xy: pt2(400.0, 400.0),
        }
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
            thargoids: Vec::from([]),
            settings: Settings {
                zoom_sensitivity: 0.05,
                pan_speed: 6.0,
            },
            egui: None,
        }
    }

    #[test]
    fn spawned_thargoids_have_100hp() {
        let game_state = new_mock_game_state();
        let test_thargoid = Thargoid::spawn(&game_state);
        assert_eq!(test_thargoid.hp, 100);
    }

    #[test]
    fn thargoids_spawn_from_top_right() {
        let game_state = new_mock_game_state();
        let test_thargoid = Thargoid::spawn(&game_state);
        assert_eq!(test_thargoid.xy.x, 400.0);
    }
}
