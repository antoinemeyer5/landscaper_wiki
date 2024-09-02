use egui::Ui;

use crate::{plant::Plant, AppLandscaperWiki};

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    egui::SidePanel::left("panel_left")
        .resizable(false)
        .exact_width(250.0)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("plants");
            });

            for plant in app.plants.iter() {
                let button = Plant::display(plant, ui);
                if button.clicked() {
                    button.highlight();
                    app.selected_plant.name = plant.name.clone();
                    app.selected_plant.notes = plant.notes.clone();
                }
            }
        });
}
