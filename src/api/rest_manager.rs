use crate::api;
use crate::errors::{Error, Result};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RestManagerConfig {
    pub timeout: Duration,
    pub base_url: String,
    pub user_agent: String,
}

impl RestManagerConfig {
    pub fn new(timeout: u64, base_url: impl AsRef<str>, user_agent: impl AsRef<str>) -> Self {
        Self {
            timeout: Duration::from_secs(timeout),
            base_url: base_url.as_ref().to_string(),
            user_agent: user_agent.as_ref().to_string(),
        }
    }
}

impl Default for RestManagerConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            base_url: "https://api.clashofclans.com/v1".to_string(),
            user_agent: format!("clash-forge/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RestManager {
    http_client: reqwest::Client,
    config: RestManagerConfig,
}

impl RestManager {
    pub fn new(token: impl AsRef<str>) -> Result<Self> {
        Self::with_config(token, RestManagerConfig::default())
    }
    pub fn with_config(token: impl AsRef<str>, config: RestManagerConfig) -> Result<Self> {
        let token = token.as_ref();

        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|_| Error::InvalidToken)?;
        headers.insert(AUTHORIZATION, auth_value);
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let http_client = reqwest::Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .user_agent(&config.user_agent)
            .build()?;

        Ok(RestManager { http_client, config })
    }
    
    async fn get_data<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        if response.status().is_success() {
            let data = response.json().await?;
            Ok(data)
        } else {
            Err(Error::Api(api::errors::Error::from_response(response).await))
        }
    }

    pub(crate) async fn get<T>(&self, url: &str, parameters: Option<HashMap<String, String>>) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.config.base_url, url);
        let response = self.http_client.get(url).query(&parameters.unwrap_or_default()).send().await?;
        self.get_data(response).await
    }

    pub(crate) async fn post<T, K>(&self, url: &str, json: &T) -> Result<K>
    where
        T: serde::Serialize,
        K: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.config.base_url, url);
        let response = self.http_client.post(url).json(json).send().await?;
        self.get_data(response).await
    }
}
