use egui::Ui;

use crate::models::{MyApp, Plant};

pub fn display(app: &mut MyApp, ui: &mut Ui) {
    egui::SidePanel::left("left_panel")
        .resizable(false)
        .default_width(150.0)
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
                        // label
                        ui.label(format!("{} {}", plant.emoji, plant.name));

                        // buttons
                        let button_infos = ui.button("ℹ");
                        let button_remove = ui.button("➖");

                        // details
                        if button_infos.clicked() {
                            app.details = (String::from("plant"), plant.id);
                        };
                        // remove this plant
                        if button_remove.clicked() {
                            delete = true;
                            app.details = (String::from("nothing"), 0); // reset
                        }
                    });
                    ui.separator();
                    !delete
                });

                // ADD
                ui.horizontal(|ui| {
                    // select
                    egui::ComboBox::from_label("")
                        .width(20.0)
                        .selected_text(format!("{:?}", app.new_plant.0))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.new_plant.0, String::from("🌲"), "Tree");
                            ui.selectable_value(&mut app.new_plant.0, String::from("🏠"), "House");
                            ui.selectable_value(
                                &mut app.new_plant.0,
                                String::from("🚜"),
                                "Tractor",
                            );
                            ui.selectable_value(&mut app.new_plant.0, String::from("🍎"), "Apple");
                            ui.selectable_value(
                                &mut app.new_plant.0,
                                String::from("🍓"),
                                "Strawberry",
                            );
                            ui.selectable_value(&mut app.new_plant.0, String::from("🍇"), "Grape");
                        });

                    // new name
                    egui::TextEdit::singleline(&mut app.new_plant.1).desired_width(60.).char_limit(8).show(ui);


                    if ui.button("➕").clicked() {
                        let new_plant = Plant::new(
                            app.plants.len(),
                            &app.new_plant.0.as_str(),
                            &app.new_plant.1.as_str(),
                            1,
                            1.,
                        );
                        app.plants.push(new_plant); // add new plant
                                                    // clean
                        app.new_plant.0 = String::from("🍇");
                        app.new_plant.1 = String::from("Grape");
                    }
                });
            });
        });
}
