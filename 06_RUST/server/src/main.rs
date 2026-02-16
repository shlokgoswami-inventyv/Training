pub mod model;
pub mod handler;
pub mod api;
pub mod routes;

use std::sync::Arc;
use tokio::sync::RwLock;

use model::User;
use routes::create_router;

pub type SharedState = Arc<RwLock<Vec<User>>>;

#[tokio::main]
async fn main() {
    let users = handler::load_users();
    let state: SharedState = Arc::new(RwLock::new(users));

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4500")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:4500");

    axum::serve(listener, app).await.unwrap();
}
