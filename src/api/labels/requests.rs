use crate::api::common::models::APIPagedResponse;
use crate::api::common::pagination::PaginationOptions;
use crate::api::labels::models::APILabel;
use crate::api::rest_manager::RestManager;
use crate::errors::Result;

macro_rules! format_url {
    ($fmt:expr, $($arg:expr)*) => {
        format!("labels/{}", format!($fmt, $($arg)*))
    };
}


impl RestManager {
    /// Retrieves a full list of player labels.
    /// 
    /// # Arguments
    /// * `pagination_options` - Optional parameters for pagination.
    /// 
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APILabel>` if successful; if an error occurs, it contains an `Error`.
    pub async fn player_labels(&self, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APILabel>> {
        let url = format_url!("players",);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }

    /// Retrieves a full list of clan labels.
    ///
    /// # Arguments
    /// * `pagination_options` - Optional parameters for pagination.
    ///
    /// # Returns
    /// `Result` containing a `APIPagedResponse<APILabel>` if successful; if an error occurs, it contains an `Error`.
    pub async fn clan_labels(&self, pagination_options: PaginationOptions) -> Result<APIPagedResponse<APILabel>> {
        let url = format_url!("clans",);
        self.get(&url, pagination_options.to_query_parameters().into()).await
    }
}