// the validator automatically applies rules to option fields only when they contain a value some so the same functions work for both createuser and updateuser without any duplication.

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/// A user as stored in the database and returned by the API.
/// `Serialize` allows conversion to JSON for responses.
/// `Deserialize` is used when we need to accept a full user (though currently not needed).
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i32,
}

pub const MIN_AGE: i32 = 16;
pub const MAX_AGE: i32 = 88;
pub const MAX_NAME_LEN: usize = 100;

/// Payload for creating a new user.
/// `id` is intentionally omitted – the database generates it automatically.
#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateUser {
    #[validate(custom(function = "validate_name"))]
    pub name: String,

    #[validate(custom(function = "validate_age"))]
    pub age: i32,
}

/// Payload for updating a user.
/// All fields are optional (`Option`), so clients can send only the fields they want to change.
#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_update_payload"))]
pub struct UpdateUser {
    #[validate(custom(function = "validate_name"))]
    pub name: Option<String>,

    #[validate(custom(function = "validate_age"))]
    pub age: Option<i32>,
}

impl CreateUser {
    pub fn normalized(self) -> Self {
        Self {
            name: self.name.trim().to_string(),
            age: self.age,
        }
    }
}

impl UpdateUser {
    pub fn normalized(self) -> Self {
        Self {
            name: self.name.map(|name| name.trim().to_string()),
            age: self.age,
        }
    }
}

fn invalid(message: String) -> ValidationError {
    let mut error = ValidationError::new("invalid");
    error.message = Some(message.into());
    error
}

fn validate_name(name: &str) -> Result<(), ValidationError> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err(invalid("name cannot be empty or whitespace-only".into()));
    }

    if trimmed.chars().count() > MAX_NAME_LEN {
        return Err(invalid(format!(
            "name cannot exceed {MAX_NAME_LEN} characters"
        )));
    }

    if !trimmed.chars().all(is_allowed_in_name) {
        return Err(invalid(
            "name may only contain English letters, spaces, hyphens and apostrophes".into(),
        ));
    }

    if !is_wrapped_by_letters(trimmed) {
        return Err(invalid(
            "name must start and end with an English letter".into(),
        ));
    }

    if has_adjacent_separators(trimmed) {
        return Err(invalid(
            "name cannot contain consecutive spaces, hyphens or apostrophes".into(),
        ));
    }

    Ok(())
}

fn validate_age(age: &i32) -> Result<(), ValidationError> {
    if (MIN_AGE..=MAX_AGE).contains(age) {
        Ok(())
    } else {
        Err(invalid(format!(
            "age must be between {MIN_AGE} and {MAX_AGE}"
        )))
    }
}

fn validate_update_payload(payload: &UpdateUser) -> Result<(), ValidationError> {
    if payload.name.is_none() && payload.age.is_none() {
        Err(invalid(
            "at least one of name or age must be provided".into(),
        ))
    } else {
        Ok(())
    }
}

fn is_allowed_in_name(c: char) -> bool {
    c.is_ascii_alphabetic() || c == ' ' || c == '-' || c == '\''
}

fn is_separator(c: char) -> bool {
    c == ' ' || c == '-' || c == '\''
}

fn is_wrapped_by_letters(name: &str) -> bool {
    name.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
        && name.chars().last().is_some_and(|c| c.is_ascii_alphabetic())
}

fn has_adjacent_separators(name: &str) -> bool {
    name.chars()
        .zip(name.chars().skip(1))
        .any(|(a, b)| is_separator(a) && is_separator(b))
}
