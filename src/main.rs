use crate::game::game_state::game_state::{GameState, Settings};
use crate::game::ship::Ship;
use crate::ui::graphics;
use crate::ui::input::event_handling::{handle_key_presses, handle_raw_window_event};
use nannou::color::BLACK;
use nannou::event::Update;
use nannou::geom::pt2;
use nannou::{App, Frame};
use nannou_egui::Egui;
use ui::gui;

mod game;
mod ui;

const ZOOM_MAX: f64 = 2.0;
const ZOOM_MIN: f64 = 0.5;

fn main() {
    nannou::app(new_game).update(update).run();
}

fn new_game(_app: &App) -> GameState {
    let window_id = _app
        .new_window()
        .view(render)
        .raw_event(handle_raw_window_event)
        .build()
        .unwrap();

    let window = _app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    GameState {
        egui,
        ship: Ship {},
        scale: 1.0,
        transform: pt2(0.0, 0.0),
        settings: Settings {
            zoom_sensitivity: 0.05,
            pan_speed: 6.0,
        },
    }
}

pub fn update(app: &App, model: &mut GameState, update: Update) {
    handle_key_presses(&app.keys.down, model);
    gui::draw_gui(model, update);
}

pub fn render(_app: &App, _model: &GameState, frame: Frame) {
    frame.clear(BLACK);
    graphics::draw_scene(_app, _model, &frame);
    _model.egui.draw_to_frame(&frame).unwrap()
}
