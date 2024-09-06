use egui::Ui;

use crate::{plant::Plant, AppLandscaperWiki};

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    egui::TopBottomPanel::top("panel_top")
        .resizable(false)
        .exact_height(90.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("landscaper_wiki: creator and viewer of plants");
                    // utils buttons
                    ui.horizontal(|ui| {
                        ui.label("utils:");
                        // export plants
                        if ui.button("export").clicked() {
                            let export = Plant::export(&app.plants);
                            match export {
                                Ok(()) => println!("Export ok"),
                                Err(error) => println!("Export ko: {}", error),
                            }
                        }
                        // import plants
                        if ui.button("import").clicked() {
                            // reset description
                            app.selected_plant = 0;
                            // actual import
                            let import = Plant::import();
                            match import {
                                Ok(res) => {
                                    app.plants = res;
                                    println!("Import ok");
                                }
                                Err(error) => println!("Import ko: {}", error),
                            }
                        }
                        // add
                        if ui.button("add").clicked() {
                            app.popup_add.open = !app.popup_add.open;
                        };
                    });
                    // more buttons
                    ui.horizontal(|ui| {
                        ui.label("more:");
                        if ui.button("infos").clicked() {
                            app.popup_infos.open = !app.popup_infos.open;
                        };
                        if ui.button("exit").clicked() {
                            std::process::exit(0);
                        };
                    });
                });
            });
        });
}
