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
                    if ui
                        .button(format!("{} {}", plant.emoji, plant.name))
                        .hovered()
                    {
                        app.details = String::from(format!(
                            "🌱 Plant\nEmoji: {}\nName: {}",
                            plant.emoji, plant.name
                        ));
                    };

                    ui.separator();
                }

                // add
                ui.vertical_centered(|ui| {
                    ui.text_edit_singleline(&mut app.new_plant_name);
                    if ui.button("➕").clicked() {
                        let new_plant = Plant::new("🌲", app.new_plant_name.as_str());
                        app.plants.push(new_plant); // add new plant
                        app.new_plant_name = String::from(""); // clean
                    }
                });
            });
        });
}
