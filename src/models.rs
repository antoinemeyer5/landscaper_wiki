pub struct MyApp {
    pub plants: Vec<Plant>,
    pub new_plant_name: String,
    pub field: Vec<Vec<Land>>,
    pub details: String,
}

pub struct Plant {
    pub emoji: String,
    pub name: String,
}

impl Plant {
    pub fn new(emoji: &str, name: &str) -> Self {
        Self {
            emoji: String::from(emoji),
            name: String::from(name),
        }
    }
}

pub struct Land {
    pub height: f32,
    pub width: f32,
    pub plant: Plant,
}

impl Land {
    pub fn new(height: f32, width: f32) -> Self {
        Self {
            height,
            width,
            plant: Plant::new("❌", "nothing"),
        }
    }
}
