use egui::{Response, Ui};

pub struct Plant {
    pub name: String,
    pub notes: String,
}

impl Plant {
    pub fn new(name: &str, notes: &str) -> Self {
        Self {
            name: String::from(name),
            notes: String::from(notes),
        }
    }

    pub fn display(plant: &Plant, ui: &mut Ui) -> Response {
        return ui.button(format!("name: {}", plant.name));
    }
}
