use crate::game::game_state::game_state::{GameState, Settings};
use crate::game::ship::Ship;
use crate::ui::input::event_handling::{handle_key_presses, raw_window_event};
use crate::ui::{graphics, gui};
use nannou::color::BLACK;
use nannou::event::Update;
use nannou::geom::pt2;
use nannou::{App, Frame};
use nannou_egui::{egui, Egui};

mod game;
mod ui;

const ZOOM_MAX: f64 = 2.0;
const ZOOM_MIN: f64 = 0.5;

fn main() {
    nannou::app(new_game_state).update(update).run();
}

fn new_game_state(_app: &App) -> GameState {
    let window_id = _app
        .new_window()
        .view(render_app)
        .raw_event(raw_window_event)
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

pub fn render_app(_app: &App, _model: &GameState, frame: Frame) {
    frame.clear(BLACK);
    graphics::draw_scene(_app, _model, &frame);
    gui::draw_ui(_app, _model, &frame);
}

pub fn update(app: &App, model: &mut GameState, update: Update) {
    handle_key_presses(&app.keys.down, model);

    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("Sensitivity:");
        ui.add(egui::Slider::new(
            &mut model.settings.zoom_sensitivity,
            1.0..=40.0,
        ));

        // Scale slider
        ui.label("Scale:");
        ui.add(egui::Slider::new(&mut 15.0, 0.0..=1000.0));

        // Rotation slider
        ui.label("Rotation:");
        ui.add(egui::Slider::new(&mut 15.0, 0.0..=360.0));

        // Random color button
        ui.button("Random color")
    });
}
