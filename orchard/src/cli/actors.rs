use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrchardAuthKeys {
    pub public_key: String,
    pub private_key: String,
}

pub async fn generate_key_pair() -> Result<OrchardAuthKeys, AppError> {
    unimplemented!()
}
