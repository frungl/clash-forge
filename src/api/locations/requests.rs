use crate::api::common::models::APIPagedResponse;
use crate::api::common::pagination::PaginationOptions;
use crate::api::locations::models::{APIClanBuilderBaseRanking, APIClanCapitalRanking, APIClanRanking, APILocation, APIPlayerBuilderBaseRanking, APIPlayerRanking};
use crate::api::rest_manager::RestManager;
use crate::errors::Result;

macro_rules! format_url {
    ($fmt:expr, $($arg:expr)*) => {
        format!("locations/{}", format!($fmt, $($arg)*))
    };
}

impl RestManager {
    /// Retrieves a list of locations.
    ///
    /// # Arguments
    /// * `pagination_options` - The paging parameters for the request.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APILocation>` if successful; if an error occurs, it contains an `Error`.
    pub async fn locations(&self, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APILocation>> {
        let url = format_url!("",);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves information about a specific location.
    ///
    /// # Arguments
    /// * `location_id` - The ID of the location.
    ///
    /// # Returns
    /// `Result` containing the `APILocation` if successful; if an error occurs, it contains an `Error`.
    pub async fn location_info(&self, location_id: impl AsRef<str>) -> Result<APILocation> {
        let url = format_url!("{}", location_id.as_ref());
        self.get(&url, None).await
    }

    /// Retrieves player rankings for a specific location.
    ///
    /// # Arguments
    /// * `location_id` - The ID of the location.
    /// * `pagination_options` - The paging parameters for the request.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIPlayerRanking>` if successful; if an error occurs, it contains an `Error`.
    pub async fn players_rankings(&self, location_id: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIPlayerRanking>> {
        let url = format_url!("{}/rankings/players", location_id.as_ref());
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves clan rankings for a specific location.
    ///
    /// # Arguments
    /// * `location_id` - The ID of the location.
    /// * `pagination_options` - The paging parameters for the request.
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIClanRanking>` if successful; if an error occurs, it contains an `Error`.
    pub async fn clans_rankings(&self, location_id: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIClanRanking>> {
        let url = format_url!("{}/rankings/clans", location_id.as_ref());
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves player builder base rankings for a specific location.
    ///
    /// # Arguments
    /// * `location_id` - The ID of the location.
    /// * `pagination_options` - The paging parameters for the request.
    /// 
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIPlayerBuilderBaseRanking>` if successful; if an error occurs, it contains an `Error`.
    pub async fn players_builder_base_rankings(&self, location_id: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIPlayerBuilderBaseRanking>> {
        let url = format_url!("{}/rankings/players-builder-base", location_id.as_ref());
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves clan builder base rankings for a specific location.
    ///
    /// # Arguments
    /// * `location_id` - The ID of the location.
    /// * `pagination_options` - The paging parameters for the request.
    /// 
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIClanBuilderBaseRanking>` if successful; if an error occurs, it contains an `Error`.
    pub async fn clans_builder_base_rankings(&self, location_id: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIClanBuilderBaseRanking>> {
        let url = format_url!("{}/rankings/clans-builder-base", location_id.as_ref());
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves clan capital rankings for a specific location.
    ///
    /// # Arguments
    /// * `location_id` - The ID of the location.
    /// * `pagination_options` - The paging parameters for the request.
    /// 
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APIClanCapitalRanking>` if successful; if an error occurs, it contains an `Error`.
    pub async fn capitals_rankings(&self, location_id: impl AsRef<str>, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APIClanCapitalRanking>> {
        let url = format_url!("{}/rankings/capitals", location_id.as_ref());
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }
}