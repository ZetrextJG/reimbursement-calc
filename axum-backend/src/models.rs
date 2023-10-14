use std::str::FromStr;

use chrono::prelude::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use strum::EnumString;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, EnumString)]
pub enum Role {
    User,
    Manager,
    Admin,
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        // HACK: This might change later
        Role::from_str(value.as_str()).unwrap_or(Role::User)
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub mail: String,
    pub username: String,
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    pub role: Role,
    pub verified: bool,
    #[serde(rename = "verificationCode")]
    pub verification_code: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    #[serde(rename = "reimburstmentPercentage")]
    pub reimburstment_percentage: Decimal,
    #[serde(rename = "maxReimburstment")]
    pub max_reimburstment: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub claim_id: i32,
    pub category_id: i32,
    pub cost: Decimal,
    pub reimburstment: Decimal,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum ClaimStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub user_id: i32,
    pub status: ClaimStatus,
}
