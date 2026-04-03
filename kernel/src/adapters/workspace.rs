use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::{self, workspaces::ActiveModel};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(
    password: &str,
    hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWorkspace {
    pub name: String,
    pub description: String,
}

impl From<CreateWorkspace> for entities::workspaces::ActiveModel {
    fn from(val: CreateWorkspace) -> Self {
        ActiveModel {
            identifier: Set(Uuid::new_v4()),
            name: Set(val.name),
            description: Set(val.description),
            is_default: Set(false),
            is_hidden: Set(false),
            is_secured: Set(false),
            password_hash: Set(None),
            created_at: Set(Utc::now().fixed_offset()),
            updated_at: Set(Utc::now().fixed_offset()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWorkspace {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_default: Option<bool>,
    pub is_hidden: Option<bool>,
    pub is_secured: Option<bool>,
    /// Plain-text password to be hashed; set to Some("") to remove the password.
    pub password: Option<String>,
}
