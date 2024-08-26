use egui::Ui;

use crate::models::{Land, MyApp, Plant};

impl Land {
    // id is position
    // example: id = 23, x=2 and y=3
    pub fn new(id: usize, height: f32, width: f32) -> Self {
        Self {
            id,
            height,
            width,
            area: height * width,
            plant: Plant::new(19, "❌", "nothing", 0, 0.0),
        }
    }

    pub fn details(app: &mut MyApp, ui: &mut Ui, x: usize, y: usize) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("ℹ");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label("🌾 Land");
                    let land = &mut app.field[y][x];
                    ui.add(
                        egui::DragValue::new(&mut land.height)
                            .prefix("Height: ")
                            .range(10..=300)
                            .suffix("m"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut land.width)
                            .prefix("Width: ")
                            .range(10..=300)
                            .suffix("m"),
                    );
                    land.area = land.height * land.width; // update area
                    ui.label(format!("Area: {}m²", land.area));
                    ui.label(format!("Plant: {}{}", land.plant.emoji, land.plant.name));

                    // test
                    // select
                    /*egui::ComboBox::from_label("")
                        .width(20.0)
                        .selected_text(format!("{:?}", selected))
                        .show_ui(ui, |ui| {
                            for plant in &app.plants {
                                ui.selectable_value(&mut selected, plant.emoji.clone(), plant.name.clone());
                            }
                        });
                    let b = ui.button("Select");
                    if b.clicked() {
                        land.plant = Plant::new(45, "🏴‍☠️", "AAA", 2, 3.4);
                    }*/
                });
            });
    }
}
