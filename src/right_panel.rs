use egui::Ui;

use crate::models::{Land, MyApp, Plant};

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    match app.details.0.as_str() {
        "plant" => Plant::details(app, ui, app.details.1),
        "land" => {
            let id = app.details.1;
            let x = id % 10;
            let y = id / 10;
            Land::details(app, ui, x, y);
        }
        _ => {
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("ℹ");
                    });
                });
        }
    }
}
