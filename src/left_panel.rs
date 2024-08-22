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
                // display plants that we didn't delete
                app.plants.retain(|plant| {
                    // keep or remove plant
                    let mut delete = false;

                    // draw
                    ui.horizontal(|ui| {
                        let button = ui.button(format!("{} {}", plant.emoji, plant.name));
                        let remove = ui.button("➖");

                        // details
                        if button.hovered() {
                            app.details = (String::from("plant"), plant.id);
                        };
                        // remove this plant
                        if remove.clicked() {
                            delete = true;
                        }
                    });
                    ui.separator();
                    !delete
                });

                // add
                ui.horizontal(|ui| {
                    ui.add_sized(
                        [100., 20.],
                        egui::TextEdit::singleline(&mut app.new_plant_name).char_limit(8),
                    );

                    if ui.button("➕").clicked() {
                        let new_plant = Plant::new(app.plants.len(), "🌲", &app.new_plant_name.as_str(), 1, 1.);
                        app.plants.push(new_plant); // add new plant
                        app.new_plant_name = String::from(""); // clean
                    }
                });
            });
        });
}
