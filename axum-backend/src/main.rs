pub mod config;
pub mod email;
pub mod forms;
mod handlers;
mod jwt_auth;
pub mod models;
mod route;
pub mod utils;

use std::sync::Arc;

use axum::http::{header, HeaderValue, Method};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

use crate::{config::Config, route::create_router};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _ = dotenv::from_filename(".env.priv");
    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let origins = vec![
        config.frontend_origin.parse::<HeaderValue>().unwrap(),
        "http://127.0.0.1".parse::<HeaderValue>().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    let app_state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });
    let app = create_router(app_state).layer(CorsLayer::very_permissive());

    let addr = format!("0.0.0.0:{}", config.port);
    println!("Listening at {}", &addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
