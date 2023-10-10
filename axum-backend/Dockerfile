# cargo-chef and the Rust toolchain
FROM lukemathwalker/cargo-chef:latest-rust-1.72.1 AS chef
WORKDIR /app

# Planner
FROM chef AS planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# Builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build --release

# Deploy
FROM rust:1.72.1-slim AS deployment
COPY --from=builder /app/target/release/axum-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/axum-backend"]
