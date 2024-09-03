use std::sync::atomic::{AtomicUsize, Ordering};

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
}
