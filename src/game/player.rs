use crate::game::drawable::Drawable;
use console_engine::{pixel, Color, ConsoleEngine};

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Drawable for Player {
    fn draw(self: &Self, engine: &mut ConsoleEngine) -> () {
        engine.set_pxl(self.x, self.y, pixel::pxl_fg('O', Color::Green));
    }
}
