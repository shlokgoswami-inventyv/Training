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


pub struct Inventory<'a, T: DisplayItem> {
    items: HashMap<String, &'a T>,
}

impl<'a, T: DisplayItem> Inventory<'a, T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, id: String, item: &'a T) -> Result<(), InventoryError> {
        if id.is_empty() {
            return Err(InventoryError::InvalidId);
        }

        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId(id));
        }

        self.items.insert(id, item);
        Ok(())
    }

    
    pub fn get_item(&self, id: &str) -> Result<&'a T, InventoryError> {
        if let Some(item) = self.items.get(id) {
            Ok(*item)
        } else {
            Err(InventoryError::MissingId(id.to_string()))
        }
    }

    pub fn remove_item(&mut self, id: &str) -> Result<&'a T, InventoryError> {
        if let Some(item) = self.items.remove(id) {
            Ok(item)
        } else {
            Err(InventoryError::MissingId(id.to_string()))
        }
    }

   
    pub fn display_all(&self) -> String {
        let format_line = |id: &String, item: &T| {
            format!("{} => {}", id, item.display())
        };

        let mut output = String::new();

        for (id, item) in &self.items {
            output.push_str(&format_line(id, item));
            output.push('\n');
        }

        output
    }
}

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
    let p1 = Product {
        name: "Keyboard".into(),
        price: 49.99,
    };

    let p2 = Product {
        name: "Mouse".into(),
        price: 19.99,
    };

    let mut inv = Inventory::new();

    inv.add_item("p1".into(), &p1).unwrap();
    inv.add_item("p2".into(), &p2).unwrap();

    println!("{}", inv.display_all());
}