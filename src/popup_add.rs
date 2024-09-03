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

            // add new plant
            ui.vertical_centered(|ui| {
                if ui.button("add").clicked() {
                    let new_name = &app.new_plant.name;
                    let new_notes = &app.new_plant.notes;
                    if new_name != "" && new_notes != "" {
                        app.plants.push(Plant::new(new_name, new_notes));
                    }
                }
            });
        });
}
