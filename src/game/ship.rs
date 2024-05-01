use nannou::color::STEELBLUE;
use nannou::prelude::pt2;
use nannou::Draw;

use crate::game::drawable::Drawable;
use crate::game::game_state::game_state::GameState;

pub struct Ship {}

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

#[cfg(test)]
mod tests {
    use crate::game::ship::Ship;

    #[test]
    fn ship_is_drawable() {
        let s = &mut Ship {};
    }
}
