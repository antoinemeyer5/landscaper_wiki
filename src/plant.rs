use egui::Ui;

use crate::models::{MyApp, Plant};

impl Plant {
    pub fn new(id: usize, emoji: &str, name: &str, quantity: u32, price: f32) -> Self {
        Self {
            id,
            emoji: String::from(emoji),
            name: String::from(name),
            quantity,
            price,
        }
    }

    pub fn details(app: &mut MyApp, ui: &mut Ui, id: usize) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("ℹ");
                });
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let index = app.plants.iter().position(|p| p.id == id).unwrap();
                    ui.label("🌱 Plant");
                    ui.label(format!("Emoji: {}", app.plants[index].emoji));
                    ui.label(format!("Name: {}", app.plants[index].name));
                    ui.add(
                        egui::DragValue::new(&mut app.plants[index].quantity)
                            .prefix("Quantity: ")
                            .range(0..=10)
                            .suffix(" (per m²)"),
                    );
                    ui.add(
                        egui::DragValue::new(&mut app.plants[index].price)
                            .prefix("Price: ")
                            .range(0..=10)
                            .suffix("$ (per unit)"),
                    );
                });
            });
    }
}
