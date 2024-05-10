use crate::game::entities::ship::Ship;
use crate::game::entities::thargoid::{PhysicsObject, Thargoid};
use crate::game::game_state::game_state::{GameState, Settings};
use crate::ui::graphics;
use crate::ui::input::event_handling::{handle_key_presses, handle_raw_window_event};
use nannou::color::BLACK;
use nannou::event::Update;
use nannou::geom::{pt2, Vec2};
use nannou::{App, Frame, LoopMode};
use nannou_egui::Egui;
use ui::gui;

mod game;
mod ui;

const ZOOM_MAX: f64 = 2.0;
const ZOOM_MIN: f64 = 0.5;

fn main() {
    nannou::app(new_game)
        .loop_mode(LoopMode::rate_fps(60.0))
        .update(update_model)
        .run();
}

fn new_game(_app: &App) -> GameState {
    let window_id = _app
        .new_window()
        .view(render_game)
        .raw_event(handle_raw_window_event)
        .build()
        .unwrap();

    let window = _app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    GameState {
        egui: Some(egui),
        ship: Ship::new(),
        thargoids: Vec::from([Thargoid {
            impulse: Vec2::new(5.0, 5.0),
            hp: 100,
            xy: pt2(500.0, 500.0),
            speed: 0.50,
        }]),
        scale: 1.0,
        transform: pt2(0.0, 0.0),
        settings: Settings {
            zoom_sensitivity: 0.05,
            pan_speed: 6.0,
        },
    }
}

pub fn update_model(app: &App, model: &mut GameState, update: Update) {
    handle_key_presses(&app.keys.down, model);
    gui::draw_gui(model, update);

    // Handle moving the elements accordingly
    for mut thargoid in &mut model.thargoids {
        thargoid.apply_next_frame()
    }
}

pub fn render_game(_app: &App, _model: &GameState, frame: Frame) {
    frame.clear(BLACK);
    graphics::draw_scene(_app, _model, &frame);
    match &_model.egui {
        Some(ref egui) => egui.draw_to_frame(&frame).unwrap(),
        None => {
            panic!("Egui wasn't initialized in handle event")
        }
    }
}
