use egui::Ui;

use crate::AppLandscaperWiki;

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("description");
            // select plant by id
            let select_id = app.selected_plant;
            if select_id != 0 {
                ui.vertical(|ui| {
                    let plant = app
                        .plants
                        .iter()
                        .filter(|p| p.id == select_id)
                        .next()
                        .unwrap();
                    ui.label(format!("id: {}", plant.id));
                    ui.label(format!("name: {}", plant.name));
                    ui.label(format!("notes: {}", plant.notes));
                    ui.label(format!("price: {} $/lbs", plant.price));
                });
                // remove
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    if ui.button("remove").clicked() {
                        app.plants.retain(|value| value.id != select_id);
                        app.selected_plant = 0; // clean description
                    }
                });
            } else {
                ui.label("display plant details by clicking on the name on the left");
            }
        });
    });
}
