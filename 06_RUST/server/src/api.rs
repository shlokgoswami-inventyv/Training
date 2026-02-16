use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use uuid::Uuid;

use crate::{model::User, handler::save_users, SharedState};

pub async fn get_users(
    State(state): State<SharedState>,
) -> Json<Vec<User>> {
    let users = state.read().await;
    Json(users.clone())
}

pub async fn get_user(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.read().await;

    users.iter()
        .find(|u| u.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn add_user(
    State(state): State<SharedState>,
    Json(mut user): Json<User>,
) -> Json<User> {
    user.id = Uuid::new_v4().to_string();

    let mut users = state.write().await;
    users.push(user.clone());

    save_users(&users);

    Json(user)
}

pub async fn update_user(
    Path(id): Path<String>,
    State(state): State<SharedState>,
    Json(updated): Json<User>,
) -> StatusCode {
    let mut users = state.write().await;

    if let Some(user) = users.iter_mut().find(|u| u.id == id) {
        user.username = updated.username;
        user.email = updated.email;
        user.address = updated.address;
        user.profile = updated.profile;

        save_users(&users);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn delete_user(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> StatusCode {
    let mut users = state.write().await;

    if users.iter().any(|u| u.id == id) {
        users.retain(|u| u.id != id);
        save_users(&users);
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}


pub async fn get_users_by_city(
    Path(city): Path<String>,
    State(state): State<SharedState>,
) -> Json<Vec<User>> {
    let users = state.read().await;

    let filtered: Vec<User> = users.iter()
        .filter(|u| u.address.city.eq_ignore_ascii_case(&city))
        .cloned()
        .collect();

    Json(filtered)
}

pub async fn get_users_by_country(
    Path(country): Path<String>,
    State(state): State<SharedState>,
) -> Json<Vec<User>> {
    let users = state.read().await;

    let filtered: Vec<User> = users.iter()
        .filter(|u| u.address.country.eq_ignore_ascii_case(&country))
        .cloned()
        .collect();

    Json(filtered)
}

pub async fn get_users_by_age(
    Path(age): Path<u32>,
    State(state): State<SharedState>,
) -> Json<Vec<User>> {
    let users = state.read().await;

    let filtered: Vec<User> = users.iter()
        .filter(|u| u.profile.age == age)
        .cloned()
        .collect();

    Json(filtered)
}


pub async fn search_users_by_username(
    Path(username): Path<String>,
    State(state): State<SharedState>,
) -> Json<Vec<User>> {
    let users = state.read().await;

    let filtered: Vec<User> = users.iter()
        .filter(|u| u.username.to_lowercase().contains(&username.to_lowercase()))
        .cloned()
        .collect();

    Json(filtered)
}

pub async fn get_user_count(
    State(state): State<SharedState>,
) -> Json<usize> {
    let users = state.read().await;
    Json(users.len())
}
