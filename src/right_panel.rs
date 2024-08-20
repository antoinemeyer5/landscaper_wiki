use egui::Ui;

use crate::models::MyApp;

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .default_width(150.0)
        .width_range(80.0..=200.0)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("ℹ");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(format!("{}", &app.details));
            });
        });
}
