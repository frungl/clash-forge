use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIPagedResponse<T> {
    pub items: Vec<T>,
    pub paging: APIPaging
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIPaging {
    pub cursors: APICursors
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APICursors {
    pub after: Option<String>,
    pub before: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIIcon {
    pub small: String,
    pub tiny: Option<String>,
    // unranked leagues do not have a medium icon
    pub medium: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct APIBadge {
    pub small: String,
    pub medium: String,
    pub large: String,
}