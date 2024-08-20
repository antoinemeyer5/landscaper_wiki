use egui::{Color32, Ui};

use crate::models::MyApp;

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Fields");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            // display field
            for row in &app.field {
                ui.horizontal(|ui| {
                    for land in row {
                        let text = egui::RichText::new(
                            format!("Land number {} ℹ", land.id))
                                .heading()
                                .color(
                                    egui::Color32::from_rgb(0, 255, 255)
                                );
                        let label = ui.button(text);
                        if label.hovered() {
                            app.details = land.details.clone();
                        }
                    }
                });
            }
        });
    });
}
