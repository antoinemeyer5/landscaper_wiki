use egui::Ui;

use crate::AppLandscaperWiki;

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    egui::TopBottomPanel::top("panel_top")
        .resizable(false)
        .exact_height(90.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("landscaper_wiki: creator and viewer of plants");
                    ui.horizontal(|ui| {
                        ui.label("utils:");
                        ui.button("import"); // TODO
                        ui.button("export"); // TODO
                        if ui.button("add").clicked() {
                            app.popup_add.open = true;
                        };
                    });
                    ui.horizontal(|ui| {
                        ui.label("more:");
                        if ui.button("infos").clicked() {
                            app.popup_infos.open = true;
                        };
                        if ui.button("exit").clicked() {
                            std::process::exit(0);
                        };
                    });
                });
            });
        });
}
