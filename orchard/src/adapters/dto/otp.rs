use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OtpKind {
    AccountVerification,
    PasswordReset,
    PasswordUpdate,
}

impl Display for OtpKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OtpKind::AccountVerification => write!(f, "account_verification"),
            OtpKind::PasswordReset => write!(f, "password_reset"),
            OtpKind::PasswordUpdate => write!(f, "password_update"),
        }
    }
}
