use axum::{routing::get, Router};

use crate::{handlers, AppState};

#[inline]
pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/ping", get(handlers::health))
        .route("/users_count", get(handlers::users_count))
        .route("/users", get(handlers::users))
        .route("/register_user", get(handlers::register_user))
        .route("/categories", get(handlers::categories))
        .with_state(app_state)
}
