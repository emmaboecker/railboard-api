use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Error, ToSchema)]
#[error("Zugportal returned an error.")]
#[serde(rename_all = "camelCase")]
pub struct ZugportalError {
    pub status_code: u32,
    pub message: String,
}

#[derive(Error, Debug)]
pub enum ZugportalOrRequestError {
    #[error("The Ris request through Zugportal returned an error.")]
    ZugportalError(#[from] ZugportalError),
    #[error("There was nothing found with these parameters")]
    NotFoundError,
    #[error(transparent)]
    FailedRequest(#[from] reqwest::Error),
}
