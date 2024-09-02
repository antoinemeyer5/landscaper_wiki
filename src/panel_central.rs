use egui::Ui;

use crate::AppLandscaperWiki;

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("description");

            let plant = &app.selected_plant;
            if plant.name != "void" {
                ui.label(format!("name: {}", plant.name));
                ui.label(format!("notes: {}", plant.notes));
            } else {
                return;
            }
        });
    });
}
