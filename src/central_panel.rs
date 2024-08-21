use egui::Ui;

use crate::models::{Land, MyApp};

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Field (= Lands)");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            // display field
            for row in &app.field {
                ui.horizontal(|ui| {
                    for land in row {
                        let button =
                            ui.add_sized([land.width, land.height], egui::Button::new("🌾"));
                        if button.hovered() {
                            app.details = Land::details(land);
                        }
                    }
                });
            }
        });
    });
}
