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
    selected_plant: usize,
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
                Plant::new("Apple", "Great round fruit", 5.99),
                Plant::new("Tomato", "Delicious edible berry", 14.26),
                Plant::new("Rice", "Amazing cereal grain", 2.34),
            ],
            popup_add: PopUp::new("add", false),
            popup_infos: PopUp::new("infos", false),
            new_plant: Plant::reset(),
            selected_plant: 0,
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
    // options
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
        ..eframe::NativeOptions::default()
    };
    // runner
    eframe::run_native(
        AppLandscaperWiki::name(),
        native_options,
        Box::new(|_| Ok(Box::<AppLandscaperWiki>::default())),
    )
}
