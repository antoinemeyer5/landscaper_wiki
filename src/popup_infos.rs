use egui::{Align2, Ui, Window};

use crate::AppLandscaperWiki;

pub fn display(app: &mut AppLandscaperWiki, ui: &mut Ui) {
    Window::new(app.popup_infos.name.clone())
        .open(&mut app.popup_infos.open)
        .pivot(Align2::RIGHT_BOTTOM)
        .show(ui.ctx(), |ui| {
            ui.label("Developed by Antoine Meyer");
            ui.label("Written in *rust-lang* using *egui*");
        });
}
