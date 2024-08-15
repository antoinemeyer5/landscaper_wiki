use egui::Ui;

use crate::models::{MyApp, Plant};

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .default_width(150.0)
        .width_range(80.0..=200.0)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Plants");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                // display plants
                for plant in &app.plants {
                    ui.label(format!("{} - {}", plant.id, plant.name));
                    ui.separator();
                }
                // add
                ui.vertical_centered(|ui| {
                    if ui.button("Add").clicked() {
                        let new_plant = Plant {
                            id: app.plants.len(),
                            name: String::from("Carrot"),
                            details: String::from("Details"),
                        };
                        app.plants.push(new_plant);
                    }
                });
            });
        });
}
