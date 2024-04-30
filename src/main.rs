mod game;

use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use crate::game::player::Player;
use crate::game::drawable::Drawable;
use crate::game::controllable::Controllable;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    player: Player
}

fn model(_app: &App) -> Model {
    Model { player: Player { x: 0.0, y: 0.0 } }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.player.handle_input(&_app)
}

fn view(_app: &App, _model: &Model, frame: Frame){
    let draw = _app.draw();
    frame.clear(WHITE);

    _model.player.draw(&draw);

    draw.to_frame(_app, &frame).unwrap();
}