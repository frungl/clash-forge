use crate::api::common::models::APIBadge;
use crate::api::leagues::models::{APIBuilderBaseLeague, APILeague};
use serde::{Deserialize, Serialize};

// ---------- Locations ----------

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APILocation {
    pub id: i64,
    pub name: String,
    pub is_country: bool,
    pub country_code: Option<String>,
    // never used?
    pub localized_name: Option<String>,
}

// ---------- Rankings ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerRanking {
    pub tag: String,
    pub name: String,
    pub exp_level: i64,
    pub trophies: i64,
    pub attack_wins: i64,
    pub defense_wins: i64,
    pub rank: i64,
    pub previous_rank: i64,
    pub clan: Option<APIPlayerRankingClan>,
    pub league: APILeague,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerRankingClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: APIBadge
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanRanking {
    pub tag: String,
    pub name: String,
    pub location: APILocation,
    pub badge_urls: APIBadge,
    pub clan_level: i64,
    pub members: i64,
    pub clan_points: i64,
    pub rank: i64,
    pub previous_rank: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerBuilderBaseRanking {
    pub tag: String,
    pub name: String,
    pub exp_level: i64,
    pub rank: i64,
    pub previous_rank: i64,
    pub builder_base_trophies: i64,
    pub builder_base_league: APIBuilderBaseLeague,
    pub clan: Option<APIPlayerRankingClan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanBuilderBaseRanking {
    pub tag: String,
    pub name: String,
    pub location: APILocation,
    pub badge_urls: APIBadge,
    pub clan_level: i64,
    pub members: i64,
    pub rank: i64,
    pub previous_rank: i64,
    pub clan_builder_base_points: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRanking {
    pub tag: String,
    pub name: String,
    pub location: APILocation,
    pub badge_urls: APIBadge,
    pub clan_level: i64,
    pub members: i64,
    pub rank: i64,
    pub previous_rank: i64,
    pub clan_capital_points: i64,
}