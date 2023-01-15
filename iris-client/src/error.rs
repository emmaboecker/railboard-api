use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Error)]
#[error("Iris returned an error.")]
pub struct IrisError;

#[derive(Error, Debug)]
pub enum IrisOrRequestError {
    #[error("Iris returned an error.")]
    IrisError(#[from] IrisError),
    #[error("Iris returned invalid/unrecognized XML: {0}")]
    InvalidXML(#[from] serde_xml_rs::Error),
    #[error(transparent)]
    FailedRequest(#[from] reqwest::Error),
}
