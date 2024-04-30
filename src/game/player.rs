use std::collections::HashSet;
use crate::game::drawable::Drawable;
use nannou::{Draw};
use nannou::color::STEELBLUE;
use nannou::prelude::Key;
use nannou::winit::event::VirtualKeyCode;
use crate::game::controllable::Controllable;

pub struct Player {
    pub x: f32,
    pub y: f32,
}

impl Drawable for Player {
    fn draw(self: &Self, draw: &Draw) -> () {
        draw.rect().color(STEELBLUE).x_y(self.x, self.y);
    }
}

impl Controllable for Player {
    fn handle_input(&mut self, down_keys: &HashSet<Key>) -> () {
        if down_keys.contains(&VirtualKeyCode::W) {
            self.y += 6.0;
        }
        if down_keys.contains(&VirtualKeyCode::A) {
            self.x -= 6.0
        }
        if down_keys.contains(&VirtualKeyCode::D) {
            self.x += 6.0;
        }
        if down_keys.contains(&VirtualKeyCode::S) {
            self.y -= 6.0
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use nannou::event::Key;
    use nannou::winit::event::VirtualKeyCode;
    use crate::game::controllable::Controllable;
    use crate::game::player::Player;

    #[test]
    fn if_no_keys_pressed_player_doesnt_move() {
        let p = &mut Player {
            x: 0.0,
            y: 0.0,
        };
        let down_keys: HashSet<Key> = HashSet::from([]);
        p.handle_input(&down_keys);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.x, 0.0);
    }

    #[test]
    fn player_can_move_up_with_w() {
        let p = &mut Player {
            x: 0.0,
            y: 0.0,
        };
        let down_keys: HashSet<Key> = HashSet::from([VirtualKeyCode::W]);
        p.handle_input(&down_keys);
        assert_eq!(p.y, 6.0);
    }

    #[test]
    fn player_can_move_down_with_s() {
        let p = &mut Player {
            x: 0.0,
            y: 0.0,
        };
        let down_keys: HashSet<Key> = HashSet::from([VirtualKeyCode::S]);
        p.handle_input(&down_keys);
        assert_eq!(p.y, -6.0);
    }

    #[test]
    fn player_can_move_right_with_d() {
        let p = &mut Player {
            x: 0.0,
            y: 0.0,
        };
        let down_keys: HashSet<Key> = HashSet::from([VirtualKeyCode::D]);
        p.handle_input(&down_keys);
        assert_eq!(p.x, 6.0);
    }

    #[test]
    fn player_can_move_left_with_a() {
        let p = &mut Player {
            x: 0.0,
            y: 0.0,
        };
        let down_keys: HashSet<Key> = HashSet::from([VirtualKeyCode::A]);
        p.handle_input(&down_keys);
        assert_eq!(p.x, -6.0);
    }

    #[test]
    fn player_can_move_diagonally() {
        let p = &mut Player {
            x: 0.0,
            y: 0.0,
        };
        let down_keys: HashSet<Key> = HashSet::from([VirtualKeyCode::A, VirtualKeyCode::W]);
        p.handle_input(&down_keys);
        assert_eq!(p.x, -6.0);
        assert_eq!(p.y, 6.0);
    }
}
