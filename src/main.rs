use eframe::egui;

use plant::Plant;
use popup::PopUp;

mod panel_central;
mod panel_left;
mod panel_top;
mod plant;
mod popup;
mod popup_add;
mod popup_infos;

struct AppLandscaperWiki {
    plants: Vec<Plant>,
    popup_add: PopUp,
    popup_infos: PopUp,
    new_plant: Plant,
    selected_plant: Plant,
}

impl AppLandscaperWiki {
    fn name() -> &'static str {
        "landscaper_wiki"
    }
}

impl Default for AppLandscaperWiki {
    fn default() -> Self {
        Self {
            plants: vec![
                Plant::new("Apple", "Great round fruit"),
                Plant::new("Tomato", "Delicious edible berry"),
                Plant::new("Rice", "Amazing cereal grain"),
            ],
            popup_add: PopUp::new("add", false),
            popup_infos: PopUp::new("infos", false),
            new_plant: Plant::new("", ""),
            selected_plant: Plant::new("void", "void"),
        }
    }
}

impl eframe::App for AppLandscaperWiki {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.5);
        egui::CentralPanel::default().show(ctx, |ui| {
            // panels
            panel_top::display(self, ui);
            panel_left::display(self, ui);
            panel_central::display(self, ui);
            // popups
            popup_add::display(self, ui);
            popup_infos::display(self, ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        AppLandscaperWiki::name(),
        native_options,
        Box::new(|_| Ok(Box::<AppLandscaperWiki>::default())),
    )
}
