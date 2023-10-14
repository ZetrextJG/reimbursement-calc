pub mod forms;
mod handlers;
pub mod models;
pub mod utils;

use axum::routing::{get, Router};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL env variable");
    println!("{}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/ping", get(handlers::health))
        .route("/users_count", get(handlers::users_count))
        .route("/users", get(handlers::users))
        .route("/register_user", get(handlers::register_user))
        .with_state(pool);

    println!("Listening at {}", &addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
