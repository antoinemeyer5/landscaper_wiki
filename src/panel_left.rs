use egui::Ui;

use crate::{plant::Plant, AppLandscaperWiki};

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    egui::SidePanel::left("panel_left")
        .resizable(false)
        .exact_width(180.0)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("plants");
            });
            // vector with plants case
            for plant in app.plants.iter() {
                let button = Plant::display(plant, ui);
                // change selected
                if button.clicked() {
                    app.selected_plant = plant.id;
                }
            }
            // empty vector case
            if app.plants.len() == 0 {
                ui.label("try to add one plant");
            }
        });
}
