
mod item;

pub struct Inventory {
    items: Vec<item::Item>
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::new(),
        }
    }
}