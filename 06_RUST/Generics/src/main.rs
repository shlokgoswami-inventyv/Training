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
            InventoryError::DuplicateId(id) => write!(f, "Item with id '{}' already exists", id),
            InventoryError::MissingId(id) => write!(f, "Item with id '{}' not found", id),
            InventoryError::InvalidId => write!(f, "Invalid empty id provided"),
        }
    }
}

pub struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}

impl<T> Inventory<T>
where
    T: DisplayItem + Clone,
{
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }

        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId(id));
        }

        self.items.insert(id, item);
        Ok(())
    }

    pub fn remove_item(&mut self, id: &str) -> Result<T, InventoryError> {
        self.items
            .remove(id)
            .ok_or_else(|| InventoryError::MissingId(id.to_string()))
    }

    pub fn get_item(&self, id: &str) -> Result<T, InventoryError> {
        self.items
            .get(id)
            .cloned()
            .ok_or_else(|| InventoryError::MissingId(id.to_string()))
    }

    pub fn display_all(&self) -> String {
        self.items
            .iter()
            .map(|(id, item)| format!("ID: {} -> {}", id, item.display()))
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
    let mut inventory = Inventory::<Product>::new();

    inventory
        .add_item(
            "p1".to_string(),
            Product {
                name: "Keyboard".to_string(),
                price: 49.99,
            },
        )
        .unwrap();

    inventory
        .add_item(
            "p2".to_string(),
            Product {
                name: "Mouse".to_string(),
                price: 19.99,
            },
        )
        .unwrap();

    println!("{}", inventory.display_all());
}