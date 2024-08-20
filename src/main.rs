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
                    details: String::from("The carrot (Daucus carota subsp. sat\
                    ivus) is a root vegetable, typically orange in color, thoug\
                    h heirloom variants including purple, black, red, white, an\
                    d yellow cultivars exist,[2][3][4] all of which are domesti\
                    cated forms of the wild carrot, Daucus carota, native to Eu\
                    rope and Southwestern Asia.")
                },
                Plant {
                    id: 1,
                    name: String::from("Rice"),
                    details: String::from("Rice is a cereal grain and in its do\
                    mesticated form is the staple food of over half of the worl\
                    d's population, particularly in Asia and Africa.")
                }
            ],
            field: vec![
                vec![
                    Land { id: 0, size: 10, details: String::from("d") },
                    Land { id: 1, size: 10, details: String::from("d") }
                ],
                vec![
                    Land { id: 2, size: 10, details: String::from("d") },
                    Land { id: 3, size: 10, details: String::from("d") },
                    Land { id: 4, size: 10, details: String::from("a") }
                ]
            ],
            details: String::from("Empty")
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
        Box::new(|_cc|
            Ok(Box::<MyApp>::default())
        )
    )
}
