use crate::api::goldpass::models::APIGoldPassSeason;
use crate::api::rest_manager::RestManager;
use crate::errors::Result;

/// Macro to format the URL for Gold Pass-related requests.
macro_rules! format_url {
    ($fmt:expr, $($arg:expr), *) => {
        format!("goldpass/{}", format!($fmt, $($arg), *))
    };
}

impl RestManager {
    /// Retrieves the current Gold Pass season.
    /// 
    /// # Returns
    /// `Result` containing a `APIGoldPassSeason` object if successful; if an error occur, it contains an `Error`.
    pub async fn goldpass(&self) -> Result<APIGoldPassSeason> {
        let url = format_url!("seasons/current",);
        self.get(&url, None).await
    }
}