use lazy_static::lazy_static;
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
    // HACK: This is just for now
    static ref USERNAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();

    static ref LOWERCASE_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    static ref UPPERCASE_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9]").unwrap();
    static ref SPECIAL_CHARACTERS_REGEX: Regex = Regex::new(r"[\!\@\#\$\%\^\&\*\(\)\{\}\[\]\_\-\+\=]").unwrap();
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("Password too short"));
    }
    if !LOWERCASE_REGEX.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one lowercase character",
        ));
    }
    if !UPPERCASE_REGEX.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase character",
        ));
    }
    if !NUMBER_REGEX.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one number",
        ));
    }
    if !SPECIAL_CHARACTERS_REGEX.is_match(password) {
        return Err(ValidationError::new(
            "Password must contain at least one special character",
        ));
    }
    Ok(())
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignupForm {
    #[validate(email)]
    pub mail: String,
    #[validate(
        length(min = 1, message = "Can not be empty"),
        regex = "USERNAME_REGEX"
    )]
    pub username: String,
    #[validate(custom = "validate_password_strength")]
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RemindPasswordForm {
    pub username: String,
    #[validate(email(message = "Is not a proper email address"))]
    pub email: String,
}

fn validate_cost(cost: &Decimal) -> Result<(), ValidationError> {
    if *cost <= Decimal::from(0) {
        Err(ValidationError::new("Cost cannot be negative nor zero"))
    } else {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ItemForm {
    pub category_id: i32,
    #[validate(custom = "validate_cost")]
    pub cost: Decimal,
}

fn validate_items(items: &[ItemForm]) -> Result<(), ValidationError> {
    items.iter().try_for_each(|item| {
        item.validate()
            .map_err(|_| ValidationError::new("Could not validate provided items"))
    })
}

#[derive(Debug, Deserialize, Validate)]
pub struct ClaimForm {
    pub user_id: i32,
    pub auth_token: String,
    #[validate(custom = "validate_items")]
    pub items: Vec<ItemForm>,
}

fn validate_percentage(value: &Decimal) -> Result<(), ValidationError> {
    if *value < Decimal::from(0) || *value > Decimal::from(100) {
        Err(ValidationError::new("Percentage must be between 0 and 100"))
    } else {
        Ok(())
    }
}

fn validate_max_reimburstment(value: &Decimal) -> Result<(), ValidationError> {
    if *value < Decimal::from(0) {
        Err(ValidationError::new(
            "Max reimburstment cannot be negative nor zero",
        ))
    } else {
        Ok(())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CategoryForm {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(custom = "validate_percentage")]
    #[serde(rename = "reimburstmentPercentage")]
    pub reimburstment_percentage: Decimal,
    #[validate(custom = "validate_max_reimburstment")]
    #[serde(rename = "maxReimburstment")]
    pub max_reimburstment: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        validate_password_strength("pass").unwrap_err();
        validate_password_strength("NO_LOWERCASE").unwrap_err();
        validate_password_strength("no_uppercase").unwrap_err();
        validate_password_strength("NoNumber").unwrap_err();
        validate_password_strength("NoSpecialCharacter1337").unwrap_err();
        validate_password_strength("ProperP4$$word").unwrap();
    }
}
