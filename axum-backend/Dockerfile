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

ARG PROD_DB
RUN if [[ -z "${PROD_DB}" ]]; \
    then export SQLX_OFFLINE=true; \
    else export DATABASE_URL=$PROD_DB; \
    fi

RUN cargo build --release

FROM builder AS test
CMD ["cargo", "test"]

FROM debian:bookworm-slim AS integration

RUN apt update && apt install libssl3 libssl-dev

ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
ARG JWT_SECRET
ENV JWT_SECRET=$JWT_SECRET
ARG SMTP_USERNAME
ENV SMTP_USERNAME=$SMPT_USERNAME
ARG SMTP_PASSWORD
ENV SMTP_PASSWORD=$SMPT_PASSWORD

COPY --from=builder /app/target/release/axum-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/axum-backend"]

FROM debian:bookworm-slim AS deployment
WORKDIR /app

RUN apt update && apt install libssl3 libssl-dev

COPY --from=builder /app/target/release/axum-backend /app
COPY ./templates/ /app/templates

ENTRYPOINT ["/app/axum-backend"]
