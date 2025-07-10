use crate::api::clans::models::APIClanMemberRole;
use crate::api::common::models::APIBadge;
use crate::api::labels::models::APILabel;
use crate::api::leagues::models::{APIBuilderBaseLeague, APILeague};
use serde::{Deserialize, Serialize};

// ---------- Verify Token ----------

#[derive(Debug, Serialize)]
pub struct APIVerifyTokenRequest {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct APIVerifyTokenResponse {
    pub tag: String,
    pub token: String,
    pub status: String,
}

// ---------- Player Info ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayer {
    pub tag: String,
    pub name: String,
    pub town_hall_level: i64,
    pub town_hall_weapon_level: i64,
    pub exp_level: i64,
    pub trophies: i64,
    pub best_trophies: i64,
    pub war_stars: i64,
    pub attack_wins: i64,
    pub defense_wins: i64,
    pub builder_hall_level: i64,
    pub builder_base_trophies: i64,
    pub best_builder_base_trophies: i64,
    // if player doesn't have a clan
    pub role: Option<APIClanMemberRole>,
    // if player doesn't have a clan
    pub war_preference: Option<APIWarPreference>,
    pub donations: i64,
    pub donations_received: i64,
    pub clan_capital_contributions: i64,
    // if player doesn't have a clan
    pub clan: Option<APIPlayerClan>,
    pub league: APILeague,
    pub builder_base_league: APIBuilderBaseLeague,
    // if player never was in legend league
    pub legend_statistics: Option<APIPlayerLegendStatistics>,
    pub achievements: Vec<APIPlayerAchievementProgress>,
    // if player doesn't change default house
    pub player_house: Option<APIPlayerHouse>,
    pub labels: Vec<APILabel>,
    pub troops: Vec<APIPlayerItemLevel>,
    pub heroes: Vec<APIPlayerItemLevel>,
    pub hero_equipment: Vec<APIPlayerItemLevel>,
    pub spells: Vec<APIPlayerItemLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIWarPreference {
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "in")]
    In,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerClan {
    pub tag: String,
    pub name: String,
    pub clan_level: i64,
    pub badge_urls: APIBadge
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerLegendStatistics {
    pub legend_trophies: i64,
    pub previous_season: Option<APILegendLeagueTournamentSeasonResult>,
    pub best_season: Option<APILegendLeagueTournamentSeasonResult>,
    pub previous_builder_base_season: Option<APILegendLeagueTournamentSeasonResult>,
    pub best_builder_base_season: Option<APILegendLeagueTournamentSeasonResult>,
    pub current_season: Option<APILegendLeagueTournamentSeasonResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APILegendLeagueTournamentSeasonResult {
    pub id: String,
    pub rank: i64,
    pub trophies: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerAchievementProgress {
    pub name: String,
    pub stars: i64,
    pub value: i64,
    pub target: i64,
    pub info: String,
    // if achievement doesn't track stats
    pub completion_info: Option<String>,
    pub village: APIVillageType
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerHouse {
    pub elements: Vec<APIPlayerHouseElement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct APIPlayerHouseElement {
    pub r#type: APIHouseElement,
    pub id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIHouseElement {
    #[serde(rename = "ground")]
    Ground,
    #[serde(rename = "walls")]
    Walls,
    #[serde(rename = "roof")]
    Roof,
    #[serde(rename = "decoration")]
    Decoration,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIPlayerItemLevel {
    pub name: String,
    pub level: i64,
    pub max_level: i64,
    pub village: APIVillageType,
    // if not active
    pub super_troop_is_active: Option<bool>,
    // only for heroes
    pub equipment: Option<Vec<APIPlayerItemLevel>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIVillageType {
    #[serde(rename = "home")]
    HomeVillage,
    #[serde(rename = "builderBase")]
    BuilderBase,
    #[serde(rename = "clanCapital")]
    ClanCapital,
}
