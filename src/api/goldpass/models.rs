use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIGoldPassSeason {
    pub start_time: String,
    pub end_time: String,
}
