use std::collections::HashMap;
use std::fmt;

pub trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Debug)]
pub enum InventoryError {
    DuplicateId(String),
    MissingId(String),
    InvalidId,
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(f, "Duplicate id: {}", id),
            Self::MissingId(id) => write!(f, "Missing id: {}", id),
            Self::InvalidId => write!(f, "Invalid id"),
        }
    }
}

pub struct Inventory<T: DisplayItem + Clone> {
    items: HashMap<String, T>,
}

impl<T: DisplayItem + Clone> Inventory<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.is_empty() {
            return Err(InventoryError::InvalidId);
        }

        if self.items.insert(id.clone(), item).is_some() {
            return Err(InventoryError::DuplicateId(id));
        }

        Ok(())
    }

    pub fn remove_item(&mut self, id: &str) -> Result<T, InventoryError> {
        self.items
            .remove(id)
            .ok_or(InventoryError::MissingId(id.to_string()))
    }

    pub fn get_item(&self, id: &str) -> Result<T, InventoryError> {
        self.items
            .get(id)
            .cloned()
            .ok_or(InventoryError::MissingId(id.to_string()))
    }

    pub fn display_all(&self) -> String {
        self.items
            .iter()
            .map(|(id, item)| format!("{} => {}", id, item.display()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Clone)]
struct Product {
    name: String,
    price: f32,
}

impl DisplayItem for Product {
    fn display(&self) -> String {
        format!("{} (${:.2})", self.name, self.price)
    }
}

fn main() {
    let mut inv = Inventory::new();

    inv.add_item("p1".into(), Product { name: "Keyboard".into(), price: 49.99 }).unwrap();
    inv.add_item("p2".into(), Product { name: "Mouse".into(), price: 19.99 }).unwrap();

    println!("{}", inv.display_all());
}
