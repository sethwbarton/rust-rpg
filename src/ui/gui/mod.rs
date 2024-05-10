use crate::game::game_state::game_state::GameState;
use nannou::event::Update;
use nannou_egui::egui;

pub fn draw_gui(model: &mut GameState, update: Update) {
    let egui = &mut model.egui;

    match egui {
        None => {
            panic!("EGUI wasn't initialized")
        }
        Some(egui) => {
            egui.set_elapsed_time(update.since_start);
            let ctx = egui.begin_frame();

            egui::TopBottomPanel::top("Info panel").show(&ctx, |ui| {
                ui.columns(3, |columns| {
                    columns[0].label("Ship Health: 100/100");
                    columns[1].label("Crew Level: 0");
                    columns[2].label("Location: Orbiting Earth");
                });
            });

            egui::TopBottomPanel::bottom("Inventory").show(&ctx, |ui| ui.label("$0"));
        }
    }
}
