use crate::game::drawable::Drawable;
use console_engine::{pixel, Color, ConsoleEngine};
use nannou::{App, Draw};
use nannou::color::STEELBLUE;

pub struct Player {
    pub x: f32,
    pub y: f32,
}

impl Drawable for Player {
    fn draw(self: &Self, draw: &Draw) -> () {
        draw.rect().color(STEELBLUE).x_y(self.x, self.y);
    }
}
