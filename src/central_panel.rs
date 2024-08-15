use egui::Ui;

use crate::models::MyApp;

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Fields");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label("Hello world!");
            // display field
            for row in &app.field {
                for land in row {
                    ui.label(format!("{}", land.id));
                }
            }
        });
    });
}
