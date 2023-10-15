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

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::User => "User".to_string(),
            Role::Manager => "Manager".to_string(),
            Role::Admin => "Admin".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BasicUserInfo {
    pub id: i32,
    pub username: String,
    pub role: Role,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,
}

impl From<User> for BasicUserInfo {
    fn from(user: User) -> Self {
        BasicUserInfo {
            id: user.id,
            username: user.username,
            role: user.role,
            verified: user.verified,
            created_at: user.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JWTokenClaims {
    /// Subscriber (user id)
    pub sub: i32,
    /// Current time
    pub iat: usize,
    /// Expiration time
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
    #[serde(rename = "reimburstmentPercentage")]
    pub reimburstment_percentage: Decimal,
    #[serde(rename = "maxReimburstment")]
    pub max_reimburstment: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub claim_id: i32,
    pub category_id: i32,
    pub cost: Decimal,
    pub reimburstment: Decimal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString)]
pub enum ClaimStatus {
    Pending,
    Accepted,
    Rejected,
}

impl From<String> for ClaimStatus {
    fn from(value: String) -> Self {
        // HACK: This might change later
        ClaimStatus::from_str(value.as_str()).unwrap_or(ClaimStatus::Pending)
    }
}

impl ToString for ClaimStatus {
    fn to_string(&self) -> String {
        match self {
            ClaimStatus::Pending => "Pending".to_string(),
            ClaimStatus::Accepted => "Accepted".to_string(),
            ClaimStatus::Rejected => "Rejected".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: i32,
    pub user_id: i32,
    pub total_cost: Option<Decimal>,
    pub reimburstment: Option<Decimal>,
    pub status: ClaimStatus,
}

mod tests {
    use super::*;

    #[test]
    fn test_role_from_string() {
        assert_eq!(Role::from("User".to_string()), Role::User);
        assert_eq!(Role::from("Manager".to_string()), Role::Manager);
        assert_eq!(Role::from("Admin".to_string()), Role::Admin);
        assert_eq!(Role::from("".to_string()), Role::User);
    }

    #[test]
    fn test_compare_roles() {
        assert!(Role::User == Role::User);
        assert!(Role::Manager == Role::Manager);
        assert!(Role::Admin == Role::Admin);

        assert!(Role::User < Role::Manager);
        assert!(Role::Manager < Role::Admin);
    }
}
