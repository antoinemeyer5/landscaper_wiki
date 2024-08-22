pub struct MyApp {
    pub plants: Vec<Plant>,
    pub new_plant_name: String,
    pub field: Vec<Vec<Land>>,
    pub details: String,
}

pub struct Plant {
    pub emoji: String,
    pub name: String,
    pub quantity: u32,
    pub price: f32,
}

impl Plant {
    pub fn new(emoji: &str, name: &str, quantity: u32, price: f32) -> Self {
        Self {
            emoji: String::from(emoji),
            name: String::from(name),
            quantity,
            price,
        }
    }

    pub fn details(plant: &Plant) -> String {
        return String::from(format!(
            "🌱 Plant\nEmoji: {}\nName: {}\nQuantity: {} (per m²)\nPrice: {}$ (per unit)",
            plant.emoji, plant.name, plant.quantity, plant.price
        ));
    }
}

pub struct Land {
    pub height: f32,
    pub width: f32,
    pub area: f32,
    pub plant: Plant,
}

impl Land {
    pub fn new(height: f32, width: f32) -> Self {
        Self {
            height,
            width,
            area: height * width,
            plant: Plant::new("❌", "nothing", 0, 0.0),
        }
    }

    pub fn details(land: &Land) -> String {
        return String::from(format!(
            "🌾 Land\nSize(h×w): {}m×{}m\nArea: {}m²\nPlant: {}{}",
            land.height, land.width, land.area, land.plant.emoji, land.plant.name
        ));
    }
}
