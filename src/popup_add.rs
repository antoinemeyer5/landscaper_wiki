use egui::{Ui, Window};

use crate::{plant::Plant, AppLandscaperWiki};

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    Window::new(app.popup_add.name.clone())
        .open(&mut app.popup_add.open)
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
            if ui.button("add").clicked() {
                app.plants
                    .push(Plant::new(&app.new_plant.name, &app.new_plant.notes));
            }
        });
}
