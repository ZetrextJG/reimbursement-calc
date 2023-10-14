use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::http::StatusCode;
use axum::{extract, Json};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::generate_random_string;
use crate::{forms, models};

pub async fn health() -> &'static str {
    "pong"
}
#[derive(Debug, Serialize)]
pub struct UsersCount {
    count: i64,
}

pub async fn users_count(
    extract::State(pool): extract::State<sqlx::PgPool>,
) -> Result<Json<UsersCount>, StatusCode> {
    let users_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);
    Ok(Json(UsersCount { count: users_count }))
}

#[derive(Debug, Deserialize)]
pub struct FetchUsers {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn users(
    extract::State(pool): extract::State<sqlx::PgPool>,
    extract::Query(query): extract::Query<FetchUsers>,
) -> Result<Json<Vec<models::User>>, StatusCode> {
    let limit = query.limit.unwrap_or(10);
    if !(0..=100).contains(&limit) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let offset = query.offset.unwrap_or(0);
    let users = sqlx::query_as!(
        models::User,
        "SELECT * FROM users LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    status: &'static str,
    message: String,
}

pub async fn register_user(
    extract::State(pool): extract::State<sqlx::PgPool>,
    extract::Json(body): extract::Json<forms::SignupForm>,
) -> Result<Json<RegisterUserResponse>, StatusCode> {
    body.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE mail = $1 OR username = $2)",
        body.mail.to_owned().to_ascii_lowercase(),
        body.username.to_owned().to_ascii_lowercase()
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::CONFLICT)?;

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .map(|hash| hash.to_string())?;

    // TODO: Implement verification email
    // let verification_code = generate_random_string(10);
    let user = sqlx::query_as!(
        models::User,
        "INSERT INTO users ( mail, username, password_hash ) VALUES ($1, $2, $3) RETURNING *",
        body.mail.to_owned().to_ascii_lowercase(),
        body.username,
        hashed_password
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = RegisterUserResponse {
        status: "success",
        // TODO: Change that to add info about email verification code
        message: format!("User {} successfully registered", user.username),
    };

    Ok(Json(response))
}

pub async fn categories(
    extract::State(pool): extract::State<sqlx::PgPool>,
) -> Result<Json<Vec<models::Category>>, StatusCode> {
    let categories = sqlx::query_as!(models::Category, "SELECT * FROM categories")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(categories))
}
