use axum::{routing::get, Router};
use crate::{SharedState, api};

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        
        .route("/users",
            get(api::get_users)
            .post(api::add_user)
        )
        .route("/users/{id}",
            get(api::get_user)
            .put(api::update_user)
            .delete(api::delete_user)
        )

       
        .route("/users/city/{city}",
            get(api::get_users_by_city)
        )
        .route("/users/country/{country}",
            get(api::get_users_by_country)
        )
        .route("/users/age/{age}",
            get(api::get_users_by_age)
        )
        .route("/users/search/{username}",
            get(api::search_users_by_username)
        )

        
        .route("/stats/count",
            get(api::get_user_count)
        )

        .with_state(state)
}
