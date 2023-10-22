use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};

use crate::{
    models::{JWTokenClaims, User},
    AppState,
};

const BEARER_START: &str = "Bearer ";
fn extract_token_from_header<B>(request: &Request<B>) -> Option<String> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with(BEARER_START) {
                let offset = BEARER_START.len();
                Some(auth_value[offset..].to_owned())
            } else {
                None
            }
        })
}

const TOKEN_NAME: &str = "re-calc-token";
pub async fn auth<B>(
    cookie_jar: CookieJar,
    State(app_state): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let token = cookie_jar
        .get(TOKEN_NAME)
        .map(|cookie| cookie.value().to_string())
        .or_else(|| extract_token_from_header(&req))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let decoding_key = DecodingKey::from_secret(app_state.config.jwt_secret.as_ref());
    let claims =
        jsonwebtoken::decode::<JWTokenClaims>(&token, &decoding_key, &Validation::default())
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .claims;

    let user: User = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&app_state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub fn create_new_auth_token(jwt_secret: String, user_id: i32) -> Result<String, StatusCode> {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: JWTokenClaims = JWTokenClaims {
        sub: user_id,
        exp,
        iat,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(token)
}

pub fn create_cookie_with_token<'a>(token: String) -> Cookie<'a> {
    Cookie::build(TOKEN_NAME, token)
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .finish()
}
pub fn create_empty_cookie<'a>() -> Cookie<'a> {
    Cookie::build(TOKEN_NAME, "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .finish()
}
