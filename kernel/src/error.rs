use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum KernelError {
    #[error("failed to connect to database due to {0}")]
    DbConnectError(String),
}
