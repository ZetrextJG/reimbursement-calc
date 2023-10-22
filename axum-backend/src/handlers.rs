use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use axum::{extract, Json};
use rand::rngs::OsRng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::forms::{ItemForm, LoginForm};
use crate::jwt_auth::{create_cookie_with_token, create_empty_cookie, create_new_auth_token};
use crate::models::{BasicUserInfo, Category, Claim, ClaimStatus, Role, User};
use crate::utils::generate_random_string;
use crate::{email, forms, AppState};

type ErrorResponse = (StatusCode, &'static str);

macro_rules! error_response {
    ($status_code: expr) => {
        ($status_code, "")
    };
    ($status_code: expr, $message: expr) => {
        ($status_code, $message)
    };
}

macro_rules! bad_request {
    ($message: expr) => {
        error_response!(StatusCode::BAD_REQUEST, $message)
    };
}

macro_rules! success_response {
    () => {
        Response::new(json!({"status": "success"}).to_string())
    };
    ($data: expr) => {
        Response::new(json!({"status": "success", "data": $data}).to_string())
    };
}

pub async fn health() -> &'static str {
    "pong"
}

const DATABASE_ERROR: ErrorResponse = error_response!(
    StatusCode::INTERNAL_SERVER_ERROR,
    "Database error. Please try again later"
);

pub async fn list_categories(
    extract::State(app_state): extract::State<Arc<AppState>>,
) -> Result<Json<Vec<Category>>, ErrorResponse> {
    let categories = sqlx::query_as!(Category, "SELECT * FROM categories")
        .fetch_all(&app_state.pool)
        .await
        .map_err(|_| DATABASE_ERROR)?;
    Ok(Json(categories))
}

#[derive(Debug, Serialize)]
pub struct UsersCount {
    count: i64,
}

pub async fn users_count(
    extract::State(app_state): extract::State<Arc<AppState>>,
) -> Result<Json<UsersCount>, ErrorResponse> {
    let users_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&app_state.pool)
        .await
        .map_err(|_| DATABASE_ERROR)?
        .unwrap_or(0);
    Ok(Json(UsersCount { count: users_count }))
}

#[derive(Debug, Deserialize)]
pub struct FetchUsers {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn users_list(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Query(query): extract::Query<FetchUsers>,
) -> Result<Json<Vec<BasicUserInfo>>, ErrorResponse> {
    let limit = query.limit.unwrap_or(10);
    if !(0..=100).contains(&limit) {
        return Err(bad_request!("Limit must be between 0 and 100"));
    }
    let offset = query.offset.unwrap_or(0);
    if offset < 0 {
        return Err(bad_request!("Offset must be greater than 0"));
    }
    let users = sqlx::query_as!(
        BasicUserInfo,
        "SELECT id, username, role, verified, created_at FROM users LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&app_state.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?;
    Ok(Json(users))
}

pub async fn register_user(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Json(body): extract::Json<forms::SignupForm>,
) -> Result<impl IntoResponse, ErrorResponse> {
    body.validate()
        .map_err(|_| bad_request!("Form failed validation"))?;

    let user_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE mail = $1 OR username = $2)",
        body.mail.to_owned().to_ascii_lowercase(),
        body.username.to_owned().to_ascii_lowercase()
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?
    .unwrap_or(false);

    if user_exists {
        return Err(error_response!(StatusCode::CONFLICT, "User already exists"));
    }

    let mut transation = app_state.pool.begin().await.map_err(|_| DATABASE_ERROR)?;

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| error_response!(StatusCode::INTERNAL_SERVER_ERROR, "Hash method failed"))
        .map(|hash| hash.to_string())?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users ( mail, username, password_hash ) VALUES ($1, $2, $3) RETURNING *",
        body.mail.to_owned().to_ascii_lowercase(),
        body.username,
        hashed_password,
    )
    .fetch_one(&mut *transation)
    .await
    .map_err(|_| DATABASE_ERROR)?;

    // Store user credentials in the database
    let verification_code = generate_random_string(10);
    let verification_url = format!(
        "{}/verifyemail/{}",
        app_state.config.frontend_origin.to_owned(),
        verification_code
    );
    // Send email verification token to the user's email address
    let email_instance =
        email::Email::new(user.clone(), verification_url, app_state.config.clone());
    email_instance.send_verification_code().await.map_err(|e| {
        println!("{:?}", e);
        error_response!(StatusCode::INTERNAL_SERVER_ERROR, "Could not send email")
    })?;

    sqlx::query!(
        "UPDATE users SET verification_code = $1 WHERE id = $2",
        verification_code,
        user.id
    )
    .execute(&mut *transation)
    .await
    .map_err(|_| DATABASE_ERROR)?;

    transation.commit().await.map_err(|_| DATABASE_ERROR)?;
    Ok(success_response!(json!({
        "message": "User created successfully. Please verify your email address"
    })))
}

pub async fn login_user(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Json(body): extract::Json<LoginForm>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user: User = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        body.username
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?
    .ok_or(bad_request!("Username or password dont match"))?;

    if !user.verified {
        return Err(error_response!(
            StatusCode::FORBIDDEN,
            "Please verify your email address"
        ));
    }

    let is_password_correct = match PasswordHash::new(&user.password_hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_password_correct {
        return Err(bad_request!("Username or password dont match"));
    }

    let token = create_new_auth_token(app_state.config.jwt_secret.clone(), user.id)
        .map_err(|status_code| error_response!(status_code))?;
    // Create a response with the access token and set it as a cookie
    let cookie = create_cookie_with_token(token.clone());

    let mut response = success_response!(json!({"token": token}));
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn verify_email(
    extract::State(data): extract::State<Arc<AppState>>,
    extract::Path(verification_code): extract::Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE verification_code = $1",
        verification_code
    )
    .fetch_optional(&data.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?
    .ok_or(error_response!(
        StatusCode::NOT_FOUND,
        "Verification code is invalid"
    ))?;

    sqlx::query!("UPDATE users SET verified = true WHERE id = $1", user.id)
        .execute(&data.pool)
        .await
        .map_err(|_| DATABASE_ERROR)?;

    Ok(success_response!(json!({
        "message": "Email verified successfully"
    })))
}

pub async fn logout_user() -> Result<impl IntoResponse, ErrorResponse> {
    let cookie = create_empty_cookie();
    let mut response = success_response!();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn users_me(
    extract::Extension(user): extract::Extension<User>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let user_info: BasicUserInfo = user.into();
    Ok(Json(user_info))
}

pub async fn users_delete_account(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Extension(user): extract::Extension<User>,
) -> Result<impl IntoResponse, ErrorResponse> {
    sqlx::query!("DELETE FROM users WHERE id = $1", user.id)
        .execute(&app_state.pool)
        .await
        .map_err(|_| DATABASE_ERROR)?;
    Ok(success_response!())
}

pub async fn make_manager(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Extension(admin): extract::Extension<User>,
    extract::Path(user_id): extract::Path<i32>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if admin.role < Role::Admin {
        return Err(error_response!(
            StatusCode::FORBIDDEN,
            "You must be an admin to perform this action"
        ));
    }
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&app_state.pool)
        .await
        .map_err(|_| DATABASE_ERROR)?
        .ok_or(error_response!(
            StatusCode::NOT_FOUND,
            "User with this id does not exist"
        ))?;
    if user.role >= Role::Manager {
        return Err(error_response!(
            StatusCode::CONFLICT,
            "User is already a manager"
        ));
    }
    sqlx::query!(
        "UPDATE users SET role = $1 WHERE id = $2",
        Role::Manager.to_string(),
        user_id
    )
    .execute(&app_state.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?;

    Ok(success_response!())
}

pub async fn create_category(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Extension(admin): extract::Extension<User>,
    extract::Json(body): extract::Json<forms::CategoryForm>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if admin.role < Role::Admin {
        return Err(error_response!(
            StatusCode::FORBIDDEN,
            "You must be an admin to perform this action"
        ));
    }

    let category = sqlx::query_as!(
        Category,
        "INSERT INTO categories ( name, reimburstment_percentage, max_reimburstment ) VALUES ($1, $2, $3) RETURNING *",
        body.name,
        body.reimburstment_percentage,
        body.max_reimburstment
    )
        .fetch_optional(&app_state.pool)
        .await
        .map_err(|_| DATABASE_ERROR)?
        .ok_or(error_response!(
            StatusCode::CONFLICT,
            "Category with this name already exists"
        ))?;

    Ok(success_response!(category))
}

pub async fn delete_category(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Extension(admin): extract::Extension<User>,
    extract::Path(category_id): extract::Path<i32>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if admin.role < Role::Admin {
        return Err(error_response!(
            StatusCode::FORBIDDEN,
            "You must be an admin to perform this action"
        ));
    }

    let category = sqlx::query_as!(
        Category,
        "DELETE FROM categories WHERE id = $1 RETURNING *",
        category_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?
    .ok_or(error_response!(
        StatusCode::NOT_FOUND,
        "Category with this id does not exist"
    ))?;

    Ok(success_response!(category))
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateCategory {
    reimburstment_percentage: Option<Decimal>,
    max_reimburstment: Option<Decimal>,
}

pub async fn update_category(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Extension(admin): extract::Extension<User>,
    extract::Path(category_id): extract::Path<i32>,
    extract::Query(updates): extract::Query<UpdateCategory>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if admin.role < Role::Admin {
        return Err(error_response!(
            StatusCode::FORBIDDEN,
            "You must be an admin to perform this action"
        ));
    }

    let category = sqlx::query_as!(
        Category,
        "UPDATE categories 
        SET reimburstment_percentage = COALESCE($1, reimburstment_percentage), max_reimburstment = COALESCE($2, max_reimburstment) 
        WHERE id = $3
        RETURNING *",
        updates.reimburstment_percentage,
        updates.max_reimburstment,
        category_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?
    .ok_or(error_response!(
        StatusCode::NOT_FOUND,
        "Category with this id does not exist"
    ))?;

    Ok(success_response!(category))
}

pub async fn create_claim(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Extension(user): extract::Extension<User>,
    extract::Json(body): extract::Json<forms::ClaimForm>,
) -> Result<impl IntoResponse, ErrorResponse> {
    body.validate()
        .map_err(|_| bad_request!("Form failed validation"))?;

    let mut transaction = app_state.pool.begin().await.map_err(|_| DATABASE_ERROR)?;

    let claim = sqlx::query_as!(
        Claim,
        "INSERT INTO claims ( user_id ) VALUES ($1) RETURNING *",
        user.id
    )
    .fetch_one(&mut *transaction)
    .await
    .map_err(|_| DATABASE_ERROR)?;

    let mut total_cost = Decimal::from(0);
    let mut reimburstment = Decimal::from(0);
    for item in body.items {
        let category = sqlx::query_as!(
            Category,
            "SELECT * FROM categories WHERE id = $1",
            item.category_id
        )
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| DATABASE_ERROR)?
        .ok_or(error_response!(
            StatusCode::NOT_FOUND,
            "Category with this id does not exist"
        ))?;

        let mut item_reimburstment =
            category.reimburstment_percentage * item.cost / Decimal::from(100);
        if item_reimburstment > category.max_reimburstment {
            item_reimburstment = category.max_reimburstment;
        }

        sqlx::query!(
            "INSERT INTO items ( claim_id, category_id, cost, reimburstment ) VALUES ($1, $2, $3, $4)",
            claim.id,
            item.category_id,
            item.cost,
            item_reimburstment
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| DATABASE_ERROR)?;

        total_cost += item.cost;
        reimburstment += item_reimburstment
    }

    sqlx::query!(
        "UPDATE claims SET total_cost = $1, reimburstment = $2 WHERE id = $3",
        total_cost,
        reimburstment,
        claim.id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| DATABASE_ERROR)?;

    transaction.commit().await.map_err(|_| DATABASE_ERROR)?;

    Ok(success_response!(claim))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToEstimate {
    pub items: Vec<ItemForm>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EstimateResult {
    pub reimburstment: Decimal,
}

pub async fn estimate_item(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Json(body): extract::Json<ItemForm>,
) -> Result<Json<EstimateResult>, ErrorResponse> {
    body.validate()
        .map_err(|_| bad_request!("Form failed validation"))?;

    let category = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE id = $1",
        body.category_id
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|_| DATABASE_ERROR)?
    .ok_or(error_response!(
        StatusCode::NOT_FOUND,
        "Invalid category for an item"
    ))?;

    let mut reimburstment = category.reimburstment_percentage * body.cost / Decimal::from(100);
    if reimburstment > category.max_reimburstment {
        reimburstment = category.max_reimburstment;
    }

    Ok(Json(EstimateResult { reimburstment }))
}

pub async fn approve_claim(
    extract::State(app_state): extract::State<Arc<AppState>>,
    extract::Extension(manager): extract::Extension<User>,
    extract::Path(claim_id): extract::Path<i32>,
    extract::Query(accept): extract::Query<bool>,
) -> Result<impl IntoResponse, ErrorResponse> {
    if manager.role < Role::Manager {
        return Err(error_response!(
            StatusCode::FORBIDDEN,
            "You must be a manager to perform this action"
        ));
    }

    let mut transaction = app_state.pool.begin().await.map_err(|_| DATABASE_ERROR)?;

    let claim = sqlx::query_as!(Claim, "SELECT * FROM claims WHERE id = $1", claim_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| DATABASE_ERROR)?
        .ok_or(error_response!(
            StatusCode::NOT_FOUND,
            "Claim with this id does not exist"
        ))?;

    if claim.status != ClaimStatus::Pending {
        return Err(error_response!(
            StatusCode::CONFLICT,
            "Claim is already processed"
        ));
    }

    let status = if accept {
        ClaimStatus::Accepted
    } else {
        ClaimStatus::Rejected
    };

    sqlx::query!(
        "UPDATE claims SET status = $1 WHERE id = $2",
        status.to_string(),
        claim_id
    )
    .execute(&mut *transaction)
    .await
    .map_err(|_| DATABASE_ERROR)?;

    transaction.commit().await.map_err(|_| DATABASE_ERROR)?;

    Ok(success_response!())
}
