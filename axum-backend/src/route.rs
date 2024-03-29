use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};

use crate::{handlers, jwt_auth::auth, AppState};

#[inline]
pub fn create_router(app_state: Arc<AppState>) -> Router {
    macro_rules! authorized {
        ($route: expr) => {
            $route.route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        };
    }

    Router::new()
        .route("/ping", get(handlers::health))
        .route("/auth/register", post(handlers::register_user))
        .route("/auth/verifyemail/:code", get(handlers::verify_email))
        .route("/auth/login", post(handlers::login_user))
        .route("/auth/logout", authorized!(get(handlers::logout_user)))
        .route("/users/count", get(handlers::users_count))
        .route("/users/list", get(handlers::users_list))
        .route("/users/startswith", get(handlers::users_startswith))
        .route("/users/me", authorized!(get(handlers::users_me)))
        .route("/users/withid", get(handlers::get_user_by_id))
        .route(
            "/users/delete_account",
            authorized!(delete(handlers::users_delete_account)),
        )
        .route(
            "/users/make_manager/:user_id",
            authorized!(get(handlers::make_manager)),
        )
        .route(
            "/categories/create",
            authorized!(post(handlers::create_category)),
        )
        .route(
            "/categories/delete/:category_id",
            authorized!(delete(handlers::delete_category)),
        )
        .route("/categories/list", get(handlers::list_categories))
        .route(
            "/categories/update/:category_id",
            patch(handlers::update_category),
        )
        .route("/claims/my", authorized!(get(handlers::list_my_claims)))
        .route(
            "/claims/pending",
            authorized!(get(handlers::list_pedning_claims)),
        )
        .route("/claims/create", authorized!(post(handlers::create_claim)))
        .route(
            "/claims/approve/:claim_id",
            authorized!(get(handlers::approve_claim)),
        )
        .route("/claims/estimate_item", post(handlers::estimate_item))
        .with_state(app_state)
}
