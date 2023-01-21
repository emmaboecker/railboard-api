use std::num::ParseIntError;

use axum::{response::IntoResponse, Json};
use iris_client::IrisOrRequestError;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use vendo_client::{VendoError, VendoOrRequestError};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RailboardApiError {
    pub domain: ErrorDomain,
    pub message: String,
    #[schema(nullable)]
    pub error: Option<UnderlyingApiError>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ErrorDomain {
    #[serde(rename = "vendo")]
    Vendo,
    #[serde(rename = "iris")]
    Iris,
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "request")]
    Request,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "errorType")]
pub enum UnderlyingApiError {
    #[serde(rename = "vendo")]
    Vendo(VendoError),
    #[serde(rename = "iris")]
    Iris,
}

pub type RailboardResult<T> = std::result::Result<T, RailboardApiError>;

impl IntoResponse for RailboardApiError {
    fn into_response(self) -> axum::response::Response {
        let code = match self.domain {
            ErrorDomain::Vendo => StatusCode::BAD_REQUEST,
            ErrorDomain::Iris => StatusCode::BAD_REQUEST,
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
                message: format!("Failed to get from Vendo: {}", err),
                error: None,
            },
            VendoOrRequestError::VendoError(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get from Vendo: {}", err),
                error: Some(UnderlyingApiError::Vendo(err)),
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
                error: Some(UnderlyingApiError::Iris),
            },
            IrisOrRequestError::InvalidXML(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get from Iris: {}", err),
                error: None,
            },
        }
    }
}
