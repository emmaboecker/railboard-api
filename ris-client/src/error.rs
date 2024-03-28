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
    #[schema(nullable)]
    pub status: Option<String>,
    #[schema(nullable)]
    pub instance_id: Option<String>,
    #[schema(nullable)]
    pub trace_id: Option<String>,
    #[schema(nullable)]
    pub span_id: Option<String>,
    #[schema(nullable)]
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

#[derive(Error, Debug)]
pub enum RisOrRequestError {
    #[error("Ris returned an error.")]
    RisError(#[from] RisError),
    #[error("The Ris request was unauthorized.")]
    RisUnauthorizedError(#[from] RisUnauthorizedError),
    #[error("There was nothing found with these parameters")]
    NotFoundError,
    #[error(transparent)]
    FailedRequest(#[from] reqwest::Error),
}
