use eframe::*;
use egui::CentralPanel;

struct MyApp {}

impl eframe::App for MyApp {
    // call every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world!");
            // more ui ...
        });
    }
}

fn main() -> eframe::Result {
    run_native(
        "landscaper",
        NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp {}))),
    )
}
