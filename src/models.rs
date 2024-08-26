pub struct MyApp {
    pub plants: Vec<Plant>,
    pub new_plant: (String, String), // emoji, name
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

pub struct Land {
    pub id: usize,
    pub height: f32,
    pub width: f32,
    pub area: f32,
    pub plant: Plant,
}
