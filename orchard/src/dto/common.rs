use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RowCount {
    pub count: u64,
}
