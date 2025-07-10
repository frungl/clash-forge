use crate::api::common::models::APIBadge;
use crate::api::labels::models::APILabel;
use crate::api::leagues::models::{APIBuilderBaseLeague, APICapitalLeague, APILeague, APIWarLeague};
use crate::api::locations::models::APILocation;
use crate::api::players::models::APIPlayerHouse;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ---------- Clan Info ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClan {
    pub tag: String,
    pub name: String,
    pub r#type: APIClanType,
    pub description: Option<String>,
    pub location: Option<APILocation>,
    pub is_family_friendly: bool,
    pub badge_urls: APIBadge,
    pub clan_level: i64,
    pub clan_points: i64,
    pub clan_builder_base_points: i64,
    pub clan_capital_points: i64,
    pub capital_league: APICapitalLeague,
    pub required_trophies: i64,
    pub war_frequency: APIWarFrequency,
    pub war_win_streak: i64,
    pub war_wins: i64,
    pub war_ties: Option<i64>,
    pub war_losses: Option<i64>,
    pub is_war_log_public: bool,
    pub war_league: APIWarLeague,
    pub members: i64,
    pub member_list: Option<Vec<APIClanMember>>,
    pub labels: Vec<APILabel>,
    pub required_builder_base_trophies: i64,
    pub required_townhall_level: i64,
    pub clan_capital: Option<APIClanCapital>,
    pub chat_language: Option<APILanguage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIClanType {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "inviteOnly")]
    InviteOnly,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIWarFrequency {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "lessThanOncePerWeek")]
    LessThanOncePerWeek,
    #[serde(rename = "oncePerWeek")]
    OncePerWeek,
    #[serde(rename = "moreThanOncePerWeek")]
    MoreThanOncePerWeek,
    #[serde(rename = "always")]
    Always,
    // only used in search
    #[serde(rename = "any")]
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanMember {
    pub tag: String,
    pub name: String,
    pub role: APIClanMemberRole,
    pub town_hall_level: i64,
    pub exp_level: i64,
    pub league: APILeague,
    pub trophies: i64,
    pub builder_base_trophies: i64,
    pub clan_rank: i64,
    pub previous_clan_rank: i64,
    pub donations: i64,
    pub donations_received: i64,
    // if player doesn't change default house it will be None
    pub player_house: Option<APIPlayerHouse>,
    pub builder_base_league: APIBuilderBaseLeague,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIClanMemberRole {
    #[serde(rename = "notMember")]
    NotMember,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "coLeader")]
    CoLeader,
    #[serde(rename = "leader")]
    Leader,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapital {
    pub capital_hall_level: i64,
    pub districts: Vec<APIClanDistrictData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanDistrictData {
    pub id: i64,
    pub name: String,
    pub district_hall_level: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APILanguage {
    pub id: i64,
    pub name: String,
    pub language_code: String,
}

// ---------- Clan War ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWar {
    pub state: APIWarState,
    pub team_size: Option<i64>,
    pub attacks_per_member: Option<i64>,
    pub battle_modifier: Option<APIBattleModifier>,
    pub preparation_start_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub clan: APIWarClan,
    pub opponent: APIWarClan,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIWarState {
    #[serde(rename = "notInWar")]
    NotInWar,
    #[serde(rename = "preparation")]
    Preparation,
    #[serde(rename = "inWar")]
    InWar,
    #[serde(rename = "warEnded")]
    WarEnded,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIBattleModifier {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "hardMode")]
    HardMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIWarClan {
    pub tag: Option<String>,
    pub name: Option<String>,
    pub badge_urls: APIBadge,
    pub clan_level: i64,
    pub attacks: Option<i64>,
    pub stars: i64,
    pub destruction_percentage: Decimal,
    pub members: Option<Vec<APIClanWarMember>>,
    pub exp_earned: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWarMember {
    pub tag: String,
    pub name: String,
    pub townhall_level: i64,
    pub map_position: i64,
    pub attacks: Option<Vec<APIClanWarAttack>>,
    pub opponent_attacks: i64,
    pub best_opponent_attack: Option<APIClanWarAttack>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWarAttack {
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: i64,
    pub destruction_percentage: i64,
    pub order: i64,
    pub duration: i64
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWarLogEntry {
    pub result: Option<APIWarResult>,
    pub end_time: String,
    pub team_size: i64,
    pub attacks_per_member: i64,
    pub battle_modifier: APIBattleModifier,
    pub clan: APIWarClan,
    pub opponent: APIWarClan,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIWarResult {
    #[serde(rename = "lose")]
    Lose,
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "tie")]
    Tie,
}

// ---------- Clan War League ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWarLeagueGroup {
    pub state: APIClanWarLeagueState,
    pub season: String,
    pub clans: Vec<APIClanWarLeagueClan>,
    pub rounds: Vec<APIClanWarLeagueRound>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIClanWarLeagueState {
    #[serde(rename = "notInWar")]
    NotInWar,
    #[serde(rename = "preparation")]
    Preparation,
    #[serde(rename = "inWar")]
    InWar,
    #[serde(rename = "ended")]
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWarLeagueClan {
    pub tag: String,
    pub name: String,
    pub clan_level: i64,
    pub badge_urls: APIBadge,
    pub members: Vec<APIClanWarLeagueClanMember>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWarLeagueClanMember {
    pub tag: String,
    #[serde(rename = "townHallLevel")]
    pub townhall_level: i64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanWarLeagueRound {
    pub war_tags: Vec<String>,
}

// ---------- Clan Capital Raid Season ----------

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeason {
    pub state: APIClanCapitalRaidSeasonState,
    pub start_time: String,
    pub end_time: String,
    pub capital_total_loot: i64,
    pub raids_completed: i64,
    pub total_attacks: i64,
    pub enemy_districts_destroyed: i64,
    pub offensive_reward: i64,
    pub defensive_reward: i64,
    pub members: Option<Vec<APIClanCapitalRaidSeasonMember>>,
    pub attack_log: Vec<APIClanCapitalRaidSeasonAttackLogEntry>,
    pub defense_log: Vec<APIClanCapitalRaidSeasonDefenseLogEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum APIClanCapitalRaidSeasonState {
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "ended")]
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeasonMember {
    pub tag: String,
    pub name: String,
    pub attacks: i64,
    pub attack_limit: i64,
    pub bonus_attack_limit: i64,
    pub capital_resources_looted: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeasonAttackLogEntry {
    pub defender: APIClanCapitalRaidSeasonClanInfo,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<APIClanCapitalRaidSeasonDistrict>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeasonDefenseLogEntry {
    pub attacker: APIClanCapitalRaidSeasonClanInfo,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<APIClanCapitalRaidSeasonDistrict>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeasonClanInfo {
    pub tag: String,
    pub name: String,
    pub level: i64,
    pub badge_urls: APIBadge,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeasonDistrict {
    pub id: i64,
    pub name: String,
    pub district_hall_level: i64,
    pub destruction_percent: i64,
    pub stars: i64,
    pub attack_count: i64,
    pub total_looted: i64,
    pub attacks: Option<Vec<APIClanCapitalRaidSeasonAttack>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeasonAttack {
    pub attacker: APIClanCapitalRaidSeasonAttacker,
    pub destruction_percent: i64,
    pub stars: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct APIClanCapitalRaidSeasonAttacker {
    pub tag: String,
    pub name: String,
}
