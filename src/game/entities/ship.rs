use nannou::color::STEELBLUE;
use nannou::prelude::pt2;
use nannou::Draw;

use crate::game::drawable::Drawable;
use crate::game::game_state::game_state::GameState;

pub struct Ship {
    pub hp: i32,
    pub turrets: Vec<Turret>,
}

pub struct Turret {}

impl Drawable for Ship {
    fn draw(self: &Self, draw: &Draw, _model: &GameState) -> () {
        let point1 = pt2(0.0, 20.0);
        let point2 = pt2(10.0, 0.0);
        let point3 = pt2(0.0, -20.0);
        let point4 = pt2(-10.0, 0.0);

        draw.quad()
            .color(STEELBLUE)
            .w(300.0)
            .h(200.0)
            .points(point1, point2, point3, point4);
    }
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            hp: 100,
            turrets: Vec::from([Turret {}, Turret {}]),
        }
    }

    pub fn hit(&mut self, points: i32) {
        self.hp -= points;
    }
}

#[cfg(test)]
mod tests {
    use crate::game::entities::ship::Ship;

    #[test]
    fn new_ship_has_100hp_2_turrets() {
        let test_ship = Ship::new();
        assert_eq!(test_ship.hp, 100);
        assert_eq!(test_ship.turrets.len(), 2)
    }

    #[test]
    fn hit_ship_to_reduce_hp() {
        let mut test_ship = Ship::new();
        test_ship.hit(50);
        assert_eq!(test_ship.hp, 50);
    }
}
