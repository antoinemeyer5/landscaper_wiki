pub struct MyApp {
    pub plants: Vec<Plant>,
    pub field: Vec<Vec<Land>>,
    pub details: String,
}

pub struct Plant {
    pub id: usize,
    pub name: String,
    pub details: String,
}

pub struct Land {
    pub id: usize,
    pub size: u32,
    pub details: String,
}
