use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Error, ToSchema)]
#[error("Ris returned an error.")]
#[serde(rename_all = "camelCase")]
pub struct RisError {
    pub error_code: i32,
    pub title: String,
    pub detail: String,
    pub status: Option<String>,
    pub instance_id: String,
    pub errors: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Error, ToSchema)]
#[error("Ris request was unauthorized.")]
#[serde(rename_all = "camelCase")]
pub struct RisUnauthorizedError {
    pub http_code: String,
    pub http_message: String,
    pub more_information: String,
}

#[derive(Serialize, Deserialize, Debug, Error, ToSchema)]
#[error("Ris request was unauthorized.")]
#[serde(rename_all = "camelCase")]
pub struct ZugportalError {
    pub status_code: u32,
    pub message: String,
}

#[derive(Error, Debug)]
pub enum RisOrRequestError {
    #[error("Ris returned an error.")]
    RisError(#[from] RisError),
    #[error("The Ris request was unauthorized.")]
    RisUnauthorizedError(#[from] RisUnauthorizedError),
    #[error("The Ris request through Zugportal returned an error.")]
    ZugportalError(#[from] ZugportalError),
    #[error(transparent)]
    FailedRequest(#[from] reqwest::Error),
}
