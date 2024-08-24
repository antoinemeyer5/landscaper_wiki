use egui::Ui;

use crate::models::MyApp;

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .min_height(150.0)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Charts");
            });

            // todo
            // display chart !!
        });
}
