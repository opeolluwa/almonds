use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]

pub struct DatabaseInsertResult {
    pub identifier: Uuid,
}
