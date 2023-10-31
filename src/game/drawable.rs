use console_engine::ConsoleEngine;

pub trait Drawable {
    fn draw(self: &Self, engine: &mut ConsoleEngine) -> ();
}
