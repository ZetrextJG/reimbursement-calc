use axum::http;
use axum::routing::{get, Router};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new().route("/ping", get(health));

    println!("Listening at {}", &addr);
    axum::Server::bind(&addr.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health() -> &'static str {
    "pong"
}
