use nannou::{Draw};

pub trait Drawable {
    fn draw(self: &Self, draw: &Draw) -> ();
}
