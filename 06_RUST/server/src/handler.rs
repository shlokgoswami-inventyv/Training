use std::fs;
use crate::model::User;

pub fn load_users() -> Vec<User> {
    fs::read_to_string("users.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

pub fn save_users(users: &[User]) {
    if let Ok(json) = serde_json::to_string_pretty(users) {
        let _ = fs::write("users.json", json);
    }
}
