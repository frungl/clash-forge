use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    pub reason: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug)]
pub struct Error {
    pub status_code: u16,
    pub reason: String,
    pub message: String,
}

impl Error {
    pub fn new(status_code: u16, reason: String, message: String) -> Self {
        Self {
            status_code,
            reason,
            message,
        }
    }

    pub async fn from_response(response: reqwest::Response) -> Self {
        let status_code = response.status();
        let data: ApiErrorResponse = response.json().await.unwrap_or(ApiErrorResponse {
            reason: None,
            message: None,
        });
        let reason = data.reason.unwrap_or_else(|| {
            match status_code {
                reqwest::StatusCode::BAD_REQUEST => "badRequest".to_string(),
                reqwest::StatusCode::FORBIDDEN => "accessDenied".to_string(),
                reqwest::StatusCode::NOT_FOUND => "notFound".to_string(),
                reqwest::StatusCode::TOO_MANY_REQUESTS => "requestThrottled".to_string(),
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => "unknownException".to_string(),
                reqwest::StatusCode::SERVICE_UNAVAILABLE => "inMaintenance".to_string(),
                reqwest::StatusCode::GATEWAY_TIMEOUT => "requestAborted".to_string(),
                _ => format!("Unspecified reason occured. Canonical reason: {}", status_code.canonical_reason().unwrap_or_default()),
            }
        });
        let message = data.message.unwrap_or_else(|| {
            match status_code {
                reqwest::StatusCode::BAD_REQUEST => "Client provided incorrect parameters for the request.".to_string(),
                reqwest::StatusCode::FORBIDDEN => "Access denied, either because of missing/incorrect credentials or used API token does not grant access to the requested resource.".to_string(),
                reqwest::StatusCode::NOT_FOUND => "Resource was not found.".to_string(),
                reqwest::StatusCode::TOO_MANY_REQUESTS => "Request was throttled, because amount of requests was above the threshold defined for the used API token.".to_string(),
                reqwest::StatusCode::INTERNAL_SERVER_ERROR => "Unknown error happened when handling the request.".to_string(),
                reqwest::StatusCode::SERVICE_UNAVAILABLE => "Service is temprorarily unavailable because of maintenance.".to_string(),
                reqwest::StatusCode::GATEWAY_TIMEOUT => "The user aborted this request.".to_string(),
                _ => "Unspecified error occurred.".to_string(),
            }
        });
        Self::new(status_code.as_u16(), reason, message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {} - {}", self.status_code, self.reason, self.message)
    }
}