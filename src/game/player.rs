use crate::game::drawable::Drawable;
use nannou::{App, Draw};
use nannou::color::STEELBLUE;
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
    fn handle_input(&mut self, _app: &App) -> () {
        if _app.keys.down.contains(&VirtualKeyCode::W) {
            self.y += 6.0;
        }
        if _app.keys.down.contains(&VirtualKeyCode::A) {
            self.x -= 6.0
        }
        if _app.keys.down.contains(&VirtualKeyCode::D) {
            self.x += 6.0;
        }
        if _app.keys.down.contains(&VirtualKeyCode::S) {
            self.y -= 6.0
        }
    }
}
