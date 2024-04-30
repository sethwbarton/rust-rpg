use nannou::{App, Draw};

pub trait Drawable {
    fn draw(self: &Self, draw: &Draw) -> ();
}
