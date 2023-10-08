use axum::routing::{get, Router};
use lazy_static::lazy_static;
use std::{fmt::format, net::SocketAddr};

//DATABASE URL
const DB_URL: &str = env!("DATABASE_URL");

// API PORT
const API_PORT: &str = env!("API_PORT");
lazy_static! {
    static ref PORT: u16 = API_PORT.parse().unwrap_or(8080);
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let address = SocketAddr::from(([0, 0, 0, 0], *PORT));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    format!("Hello world")
}
