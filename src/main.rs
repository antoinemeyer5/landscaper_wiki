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
                Plant {
                    id: 0,
                    name: String::from("Carrot"),
                    details: String::from("test"),
                },
                Plant {
                    id: 1,
                    name: String::from("Rice"),
                    details: String::from("test 2"),
                },
            ],
            field: vec![
                vec![Land { id: 0, size: 10 }, Land { id: 1, size: 10 }],
                vec![Land { id: 2, size: 10 }, Land { id: 3, size: 10 }],
            ],
            details: String::from("Empty"),
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
