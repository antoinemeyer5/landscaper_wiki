use egui::{Align2, Ui, Window};

use crate::{plant::Plant, AppLandscaperWiki};

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    Window::new(app.popup_add.name.clone())
        .open(&mut app.popup_add.open)
        .pivot(Align2::CENTER_CENTER)
        .show(ui.ctx(), |ui| {
            // new name
            ui.horizontal(|ui| {
                ui.label("name");
                ui.text_edit_singleline(&mut app.new_plant.name);
            });
            // new notes
            ui.horizontal(|ui| {
                ui.label("notes");
                ui.text_edit_singleline(&mut app.new_plant.notes);
            });
            // new price
            ui.horizontal(|ui| {
                ui.label("price");
                ui.add(egui::DragValue::new(&mut app.new_plant.price).suffix(" $/lbs"));
            });
            // add new plant
            ui.vertical_centered(|ui| {
                if ui.button("add").clicked() {
                    // create new plant
                    let new_name = &app.new_plant.name;
                    let new_notes = &app.new_plant.notes;
                    let new_price = &app.new_plant.price;
                    if new_name != "" && new_notes != "" {
                        // add new plant
                        app.plants.push(Plant::new(new_name, new_notes, *new_price));
                        // reset fields
                        app.new_plant = Plant::reset();
                    }
                }
            });
        });
}
