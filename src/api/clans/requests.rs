use crate::api::clans::models::{APIClan, APIClanCapitalRaidSeason, APIClanMember, APIClanWar, APIClanWarLeagueGroup, APIClanWarLogEntry};
use crate::api::clans::search::ClanSearchOptions;
use crate::api::common::models::APIPagedResponse;
use crate::api::common::pagination::PaginationOptions;
use crate::api::common::utils::normalize_tag;
use crate::api::rest_manager::RestManager;
use crate::errors::Result;

/// Macro to format the URL for clan-related requests.
macro_rules! format_url {
    ($fmt:expr, $($arg:expr), *) => {
        format!("clans/{}", format!($fmt, $($arg), *))
    };
}

impl RestManager {
    /// Retrieves a list of clans based on the search options.
    ///
    /// # Arguments
    /// * `search_options` - The filter parameters for searching clans.
    /// * `pagination_options` - The paging parameters for the request.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIClan>` if successful; if an error occurs, it contains an `Error`.
    pub async fn clans(&self, search_options: ClanSearchOptions, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIClan>> {
        let url = "clans".to_string();
        let mut parameters = search_options.to_query_parameters();
        parameters.extend(pagination_options.to_query_parameters());
        self.get(&url, parameters.into()).await
    }

    /// Retrieves clan information.
    ///
    /// # Arguments
    /// * `tag` - The tag of the clan.
    ///
    /// # Returns
    /// `Result` containing the `APIClan` if successful; if an error occurs, it contains an `Error`.
    pub async fn clan_info(&self, tag: impl AsRef<str>) -> Result<APIClan> {
        let tag = normalize_tag(tag.as_ref())?;
        let url = format_url!("{}", tag);
        self.get(&url, None).await
    }

    /// Retrieves list of clan members.
    ///
    /// # Arguments
    /// * `tag` - The tag of the clan.
    /// * `pagination_options` - Paging request parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIClanMember>` if successful; if an error occurs, it contains an `Error`.
    pub async fn clan_members(&self, tag: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIClanMember>> {
        let tag = normalize_tag(tag.as_ref())?;
        let url = format_url!("{}/members", tag);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves clan's clan war log.
    ///
    /// # Arguments
    /// * `tag` - The tag of the clan.
    /// * `pagination_options` - Paging request parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIClanWarLogEntry>` if successful; if an error occurs, it contains an `Error`.
    pub async fn war_log(&self, tag: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIClanWarLogEntry>> {
        let tag = normalize_tag(tag.as_ref())?;
        let url = format_url!("{}/warlog", tag);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves information about clan's current war.
    ///
    /// # Arguments
    /// * `tag` - The tag of the clan.
    ///
    /// # Returns
    /// `Result` containing the `APIClanWar` if successful; if an error occurs, it contains an `Error`.
    pub async fn current_war(&self, tag: impl AsRef<str>) -> Result<APIClanWar> {
        let tag = normalize_tag(tag.as_ref())?;
        let url = format_url!("{}/currentwar", tag);
        self.get(&url, None).await
    }

    /// Retrieves information about clan's current clan war league group.
    ///
    /// # Arguments
    /// * `tag` - The tag of the clan.
    ///
    /// # Returns
    /// `Result` containing a `APIClanWarLeagueGroup` if successful; if an error occurs, it contains an `Error`.
    pub async fn clan_war_league_group(&self, tag: impl AsRef<str>) -> Result<APIClanWarLeagueGroup> {
        let tag = normalize_tag(tag.as_ref())?;
        let url = format_url!("{}/currentwar/leaguegroup", tag);
        self.get(&url, None).await
    }

    /// Retrieves information about clan's current war in clan war league.
    ///
    /// # Arguments
    /// * `war_tag` - The tag of the war in current clan war league.
    ///
    /// # Returns
    /// `Result` containing the `APIClanWarLeagueGroup` if successful; if an error occurs, it contains an `Error`.
    pub async fn clan_war_league_war(&self, war_tag: impl AsRef<str>) -> Result<APIClanWar> {
        let war_tag = normalize_tag(war_tag.as_ref())?;
        let url = format!("clanwarleagues/wars/{war_tag}");
        self.get(&url, None).await
    }

    /// Retrieves clan's capital raid seasons.
    ///
    /// # Arguments
    /// * `tag` - The tag of the clan.
    /// * `pagination_options` - Paging request parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIClanCapitalRaidSeason>` if successful; if an error occurs, it contains an `Error`.
    pub async fn clan_capital_raid_seasons(&self, tag: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIClanCapitalRaidSeason>> {
        let tag = normalize_tag(tag.as_ref())?;
        let url = format_url!("{}/capitalraidseasons", tag);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }
}
