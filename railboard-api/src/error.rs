use std::num::ParseIntError;

use axum::{response::IntoResponse, Json};
use iris_client::IrisOrRequestError;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use vendo_client::VendoOrRequestError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RailboardApiError {
    pub domain: ErrorDomain,
    pub message: String,
    pub error: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorDomain {
    #[serde(rename = "vendo")]
    Vendo,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "request")]
    Request,
}

pub type RailboardResult<T> = std::result::Result<T, RailboardApiError>;

impl IntoResponse for RailboardApiError {
    fn into_response(self) -> axum::response::Response {
        let code = match self.domain {
            ErrorDomain::Vendo => StatusCode::BAD_REQUEST,
            ErrorDomain::Input => StatusCode::BAD_REQUEST,
            ErrorDomain::Request => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, Json(self)).into_response()
    }
}

impl From<ParseIntError> for RailboardApiError {
    fn from(value: ParseIntError) -> Self {
        RailboardApiError {
            domain: ErrorDomain::Input,
            message: format!("Required Integer but found: {}", value),
            error: None,
        }
    }
}

impl From<VendoOrRequestError> for RailboardApiError {
    fn from(value: VendoOrRequestError) -> Self {
        match value {
            VendoOrRequestError::FailedRequest(err) => RailboardApiError {
                domain: ErrorDomain::Request,
                message: format!("Failed to get station board from Vendo: {}", err),
                error: None,
            },
            VendoOrRequestError::VendoError(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get station board from Vendo: {}", err),
                error: Some(serde_json::to_value(err).unwrap()),
            },
        }
    }
}

impl From<IrisOrRequestError> for RailboardApiError {
    fn from(value: IrisOrRequestError) -> Self {
        match value {
            IrisOrRequestError::FailedRequest(err) => RailboardApiError {
                domain: ErrorDomain::Request,
                message: format!("Failed to get from Iris: {}", err),
                error: None,
            },
            IrisOrRequestError::IrisError(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get from Iris: {}", err),
                error: Some(serde_json::to_value(err).unwrap()),
            },
            IrisOrRequestError::InvalidXML(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get from Iris: {}", err),
                error: None,
            },
        }
    }
}
