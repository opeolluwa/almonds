use serde::{Deserialize, Serialize};

use crate::entities::countries;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchCountriesResponse {
    pub records: Vec<countries::Model>,
}
