use std::{
    fs::File,
    io::{Read, Write},
    sync::atomic::{AtomicUsize, Ordering},
};

use egui::{Response, Ui};

pub struct Plant {
    pub id: usize,
    pub name: String,
    pub notes: String,
    pub price: f32,
}

impl Plant {
    pub fn new(name: &str, notes: &str, price: f32) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            name: String::from(name),
            notes: String::from(notes),
            price,
        }
    }

    pub fn display(plant: &Plant, ui: &mut Ui) -> Response {
        return ui.button(format!("name: {}", plant.name));
    }

    pub fn reset() -> Plant {
        return Plant {
            id: 0,
            name: "".to_string(),
            notes: "".to_string(),
            price: 0.,
        };
    }

    // export a vector of plants into a txt file
    pub fn export(plants: &Vec<Plant>) -> std::io::Result<()> {
        let mut file = File::create("export.txt")?;
        for plant in plants.iter() {
            file.write_all(format!("{},{},{}\n", plant.name, plant.notes, plant.price).as_bytes())?;
        }
        Ok(())
    }

    // import
    pub fn import() -> std::io::Result<Vec<Plant>> {
        let mut file = File::open("import.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines = contents.lines();
        let mut plants: Vec<Plant> = Vec::new();
        for line in lines {
            let items = line.split(',').collect::<Vec<&str>>();
            plants.push(Plant::new(
                items.get(0).unwrap(),
                items.get(1).unwrap(),
                items.get(2).unwrap().parse::<f32>().unwrap(),
            ));
        }
        Ok(plants)
    }
}
