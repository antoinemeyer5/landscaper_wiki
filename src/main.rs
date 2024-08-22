use eframe::*;
use egui::CentralPanel;

mod central_panel;
mod left_panel;
mod right_panel;

mod models;
use models::{Land, MyApp, Plant};

impl Default for MyApp {
    fn default() -> Self {
        Self {
            plants: vec![
                Plant::new(0, "🍎", "Apple", 2, 2.5),
                Plant::new(1, "🍚", "Rice", 6, 1.),
            ],
            new_plant_name: String::from(""),
            field: vec![
                vec![Land::new(00, 10., 20.), Land::new(01, 100., 20.)],
                vec![
                    Land::new(10, 10., 10.),
                    Land::new(11, 40., 40.),
                    Land::new(12, 10., 50.),
                ],
            ],
            details: (String::from("Details about hovered items"), 0),
        }
    }
}

impl eframe::App for MyApp {
    // call every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            left_panel::display(self, ui);
            right_panel::display(self, ui);
            central_panel::display(self, ui);
        });
    }
}

fn main() -> eframe::Result {
    run_native(
        "landscaper",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
