use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Error, ToSchema)]
#[error("Vendo returned an error.")]
pub struct VendoError {
    pub domain: String,
    pub code: String,
    pub status: String,
}

#[derive(Error, Debug)]
pub enum VendoOrRequestError {
    #[error("Vendo returned an error.")]
    VendoError(#[from] VendoError),
    #[error(transparent)]
    FailedRequest(#[from] reqwest::Error),
}
