use crate::game::drawable::Drawable;
use crate::Model;
use nannou::color::STEELBLUE;
use nannou::prelude::pt2;
use nannou::Draw;

pub struct Ship {}

impl Drawable for Ship {
    fn draw(self: &Self, draw: &Draw, _model: &Model) -> () {
        let point1 = pt2(0.0 + _model.transform.x, 20.0 + _model.transform.y);
        let point2 = pt2(10.0 + _model.transform.x, 0.0 + _model.transform.y);
        let point3 = pt2(0.0 + _model.transform.x, -20.0 + _model.transform.y);
        let point4 = pt2(-10.0 + _model.transform.x, 0.0 + _model.transform.y);

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
