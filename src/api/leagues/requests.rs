use crate::api::common::models::APIPagedResponse;
use crate::api::common::pagination::PaginationOptions;
use crate::api::leagues::models::{APIBuilderBaseLeague, APICapitalLeague, APILeague, APILeagueSeason, APIWarLeague};
use crate::api::locations::models::APIPlayerRanking;
use crate::api::rest_manager::RestManager;
use crate::errors::Result;

macro_rules! format_url {
    ($r#type:expr, $fmt:expr, $($arg:expr),*) => {
        format!("{}leagues/{}", $r#type, format!($fmt, $($arg),*))
    };
}

impl RestManager {
    /// Retrieves a full list of leagues.
    ///
    /// # Arguments
    /// * `pagination_options` - Optional parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APILeague>` if successful; if an error occurs, it contains an `Error`.
    pub async fn leagues(&self, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APILeague>> {
        let url = format_url!("", "",);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves information about a specific league.
    ///
    /// # Arguments
    /// * `league_id` - The ID of the league.
    ///
    /// # Returns
    /// `Result` containing an `APILeague` if successful; if an error occurs, it contains an `Error`.
    pub async fn league_info(&self, league_id: impl AsRef<str>) -> Result<APILeague> {
        let url = format_url!("", "{}", league_id.as_ref());
        self.get(&url, None).await
    }

    /// Retrieves a full list of builder base leagues.
    ///
    /// # Arguments
    /// * `pagination_options` - Optional parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIBuilderBaseLeague>` if successful; if an error occurs, it contains an `Error`.
    pub async fn builder_base_leagues(&self, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIBuilderBaseLeague>> {
        let url = format_url!("builderbase", "",);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves information about a specific builder base league.
    ///
    /// # Arguments
    /// * `league_id` - The ID of the league.
    ///
    /// # Returns
    /// `Result` containing an `APIBuilderBaseLeague` if successful; if an error occurs, it contains an `Error`.
    pub async fn builder_base_league_info(&self, league_id: impl AsRef<str>) -> Result<APIBuilderBaseLeague> {
        let url = format_url!("builderbase", "{}", league_id.as_ref());
        self.get(&url, None).await
    }

    /// Retrieves a full list of league seasons for a specific league.
    ///
    /// # Arguments
    /// * `league_id` - The ID of the league.
    /// * `pagination_options` - Optional parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APILeagueSeason>` if successful; if an error occurs, it contains an `Error`.
    pub async fn league_seasons(&self, league_id: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APILeagueSeason>> {
        let url = format_url!("", "{}/seasons", league_id.as_ref());
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves a full list of player rankings for a specific league and season.
    ///
    /// # Arguments
    /// * `league_id` - The ID of the league.
    /// * `season_id` - The ID of the season.
    /// * `pagination_options` - Optional parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIPlayerRanking>` if successful; if an error occurs, it contains an `Error`.
    pub async fn league_season_rankings(&self, league_id: impl AsRef<str>, season_id: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIPlayerRanking>> {
        let url = format_url!("", "{}/seasons/{}", league_id.as_ref(), season_id.as_ref());
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves a full list of war leagues.
    ///
    /// # Arguments
    /// * `pagination_options` - Optional parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIWarLeague>` if successful; if an error occurs, it contains an `Error`.
    pub async fn war_leagues(&self, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIWarLeague>> {
        let url = format_url!("war", "",);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves information about a specific war league.
    ///
    /// # Arguments
    /// * `league_id` - The ID of the league.
    ///
    /// # Returns
    /// `Result` containing an `APIWarLeague` if successful; if an error occurs, it contains an `Error`.
    pub async fn war_league_info(&self, league_id: impl AsRef<str>) -> Result<APIWarLeague> {
        let url = format_url!("war", "{}", league_id.as_ref());
        self.get(&url, None).await
    }

    /// Retrieves a full list of `capital leagues.
    ///
    /// # Arguments
    /// * `pagination_options` - Optional parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APICapitalLeague>` if successful; if an error occurs, it contains an `Error`.
    pub async fn capital_leagues(&self, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APICapitalLeague>> {
        let url = format_url!("capital", "",);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves information about a specific capital league.
    ///
    /// # Arguments
    /// * `league_id` - The ID of the league.
    ///
    /// # Returns
    /// `Result` containing an `APICapitalLeague` if successful; if an error occurs, it contains an `Error`.
    pub async fn capital_league_info(&self, league_id: impl AsRef<str>) -> Result<APICapitalLeague> {
        let url = format_url!("capital", "{}", league_id.as_ref());
        self.get(&url, None).await
    }
}