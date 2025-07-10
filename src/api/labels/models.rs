use crate::api::common::models::APIIcon;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APILabel {
    pub id: i64,
    pub name: String,
    pub icon_urls: APIIcon,
}