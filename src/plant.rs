use std::{
    fs::File,
    io::Write,
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
            file.write_all(
                format!(
                    "{},{},{},{}\n",
                    plant.id, plant.name, plant.notes, plant.price
                )
                .as_bytes(),
            )?;
        }
        Ok(())
    }
}
