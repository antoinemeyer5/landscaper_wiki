pub struct PopUp {
    pub name: String,
    pub open: bool,
}

impl PopUp {
    pub fn new(name: &str, open: bool) -> Self {
        Self {
            name: String::from(name),
            open,
        }
    }
}
