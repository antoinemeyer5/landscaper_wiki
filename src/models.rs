use egui::Ui;

pub struct MyApp {
    pub plants: Vec<Plant>,
    pub new_plant_name: String,
    pub field: Vec<Vec<Land>>,
    pub details: (String, usize),
}

pub struct Plant {
    pub id: usize,
    pub emoji: String,
    pub name: String,
    pub quantity: u32,
    pub price: f32,
}

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

pub struct Land {
    pub id: usize,
    pub height: f32,
    pub width: f32,
    pub area: f32,
    pub plant: Plant,
}

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
                });
            });
    }
}
