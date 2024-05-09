use crate::game::game_state::game_state::GameState;
use nannou::event::Update;
use nannou_egui::egui;

pub fn draw_gui(model: &mut GameState, update: Update) {
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
