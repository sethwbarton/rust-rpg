use crate::game::drawable::Drawable;
use crate::game::game_state::game_state::GameState;
use nannou::geom::{pt2, Point2, Vec2};
use nannou::Draw;
use std::ops::Add;

pub trait PhysicsObject {
    fn apply_impulse(self: &mut Self, impulse: Vec2) -> ();
    fn apply_next_frame(self: &mut Self) -> ();
    fn get_xy(self: &Self) -> Point2;
}

pub struct Thargoid {
    pub hp: i32,
    pub xy: Point2,
    pub impulse: Vec2,
    pub speed: f32,
}

impl Drawable for Thargoid {
    fn draw(self: &Self, draw: &Draw, model: &GameState) {
        draw.xy(self.xy).ellipse().radius(20.0).resolution(6.0);
    }
}

impl PhysicsObject for Thargoid {
    fn apply_impulse(&mut self, impulse: Vec2) -> () {
        self.impulse = self.impulse.add(impulse);
    }

    fn apply_next_frame(self: &mut Self) -> () {
        self.xy = pt2(self.speed * self.impulse.x, self.speed * self.impulse.y)
    }

    fn get_xy(self: &Self) -> Point2 {
        return self.xy;
    }
}

impl Thargoid {
    pub fn spawn(state: &GameState) -> Self {
        Thargoid {
            impulse: Vec2::new(0.0, 0.0),
            hp: 100,
            xy: pt2(400.0, 400.0),
            speed: 0.50,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::entities::ship::Ship;
    use crate::game::entities::thargoid::{PhysicsObject, Thargoid};
    use crate::game::game_state::game_state::{GameState, Settings};
    use nannou::geom::{pt2, Vec2};

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

    #[test]
    fn new_thargoids_are_not_in_motion() {
        let game_state = new_mock_game_state();
        let test_thargoid = Thargoid::spawn(&game_state);
        assert_eq!(test_thargoid.impulse, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn adding_multiple_impulses() {
        let game_state = new_mock_game_state();
        let mut test_thargoid = Thargoid::spawn(&game_state);

        test_thargoid.apply_impulse(Vec2::new(5.0, 5.0));

        assert_eq!(test_thargoid.impulse, Vec2::new(5.0, 5.0));

        test_thargoid.apply_impulse(Vec2::new(-5.0, -5.0));
        assert_eq!(test_thargoid.impulse, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn apply_next_frame_moves_towards_impulses() {
        let game_state = new_mock_game_state();
        let mut test_thargoid = Thargoid::spawn(&game_state);

        // Have to force location at origin since for now we are always spawning at 400,400
        // this is easier to visualize for tests
        test_thargoid.xy = pt2(0.0, 0.0);

        test_thargoid.apply_impulse(Vec2::new(5.0, 5.0));
        test_thargoid.apply_next_frame();

        // For now, speed just represents what percentage of an applied impulse magnitude we utilize
        assert_eq!(
            test_thargoid.get_xy(),
            pt2(test_thargoid.speed * 5.0, test_thargoid.speed * 5.0)
        );
    }
}
