use eframe::*;
use egui::CentralPanel;

struct MyApp {}

impl eframe::App for MyApp {
    // call every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            //ui.label("Hello world!");

            //
            egui::TopBottomPanel::top("top_panel")
                .resizable(true)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // title
                        ui.vertical_centered(|ui| {
                            ui.heading("landscaper");
                        });

                        //
                        ui.heading("Def");
                        ui.label(
                            "Their job duties include planting \
                            seasonal flowers, trimming hedges, pruning trees, \
                            fertilizing plants, mowing grass and managing \
                            pests. Landscapers may also work with building \
                            contractors to construct garden walls, walkways \
                            and steps.",
                        );
                        ui.add_space(12.0);

                        //
                        ui.heading("Idea");
                        ui.label(
                            "Plant management application for your \
                            garden. Know when to harvest carrots, when to \
                            water tomatoes, tell us when you've planted \
                            tulips, know when to clean up strawberry cuttings, \
                            etc.",
                        );
                        ui.add_space(12.0);
                    });
                });

            egui::SidePanel::left("left_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Left Panel");
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("Hello world!");
                    });
                });

            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=200.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Right Panel");
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("Hello world!");
                    });
                });

            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Central Panel");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Hello world!");
                });
            });
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
