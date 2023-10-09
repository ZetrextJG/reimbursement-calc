use axum::{
    extract::State,
    routing::{get, Router},
    Json,
};
use axum_error::Result;
use bigdecimal::BigDecimal;
use lazy_static::lazy_static;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};

//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");

// API PORT
const API_PORT_STR: &str = env!("API_PORT");
lazy_static! {
    static ref API_PORT: u16 = API_PORT_STR.parse().unwrap_or_else(|_| {
        println!("Could not find port {}. Defaulting to 8080", API_PORT_STR);
        8080
    });
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct Item {
    id: i32,
    name: String,
    description: Option<String>,
    percent_remburst: BigDecimal,
    max_rembursement: BigDecimal,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Connecting to database at {}", DB_URL);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL)
        .await?;

    println!("Setting up the app");
    let app = Router::new()
        .route("/", get(index))
        .route("/record", get(record))
        .with_state(pool);

    println!("Starting server on port {}", *API_PORT);
    let address = SocketAddr::from(([0, 0, 0, 0], *API_PORT));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn index() -> String {
    format!("Hello world")
}

async fn record(State(pool): State<PgPool>) -> Result<Json<Vec<Item>>> {
    let item = sqlx::query_as!(
        Item,
        r#"
        SELECT * FROM items
        "#
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(item))
}
