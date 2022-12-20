use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Error)]
#[error("Vendo returned an error.")]
pub struct VendoError {
    pub domain: String,
    pub code: String,
    pub status: String,
}
