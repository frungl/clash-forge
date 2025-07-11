use crate::api::common::utils;
use crate::api::players::models::{APIPlayer, APIVerifyTokenRequest, APIVerifyTokenResponse};
use crate::api::rest_manager::RestManager;
use crate::errors::Result;

macro_rules! encode_url {
    ($fmt:expr, $($arg:expr)*) => {
        format!("players/{}", format!($fmt, $($arg)*))
    };
}

impl RestManager {
    /// Retrieves player information by their tag.
    /// 
    /// # Arguments
    /// * `tag` - The tag of the player, which should be a valid Clash of Clans player tag.
    /// 
    /// # Returns
    /// `Result` containing an `APIPlayer` if successful; if an error occurs, it contains an `Error`.
    pub async fn player(&self, tag: impl AsRef<str>) -> Result<APIPlayer> {
        let tag = utils::normalize_tag(tag.as_ref());
        let url = encode_url!("{}", tag);
        self.get(&url, None).await
    }

    /// Verifies a token for a player.
    /// 
    /// # Arguments
    /// * `tag` - The tag of the player, which should be a valid Clash of Clans player tag.
    /// * `token` - The token to verify, which should be a string.
    /// 
    /// # Returns
    /// `Result` containing a boolean indicating whether the token verification was successful (`true`) or not (`false`); if an error occurs, it contains an `Error`.
    pub async fn verify(&self, tag: impl AsRef<str>, token: impl AsRef<str>) -> Result<bool> {
        let tag = utils::normalize_tag(tag.as_ref());
        let token = token.as_ref().to_string();
        let url = encode_url!("{}/verifytoken", tag);
        let request = APIVerifyTokenRequest { token };
        self.post(&url, &request).await.map(|x: APIVerifyTokenResponse| x.status == "ok" )
    }
}