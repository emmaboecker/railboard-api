use std::num::ParseIntError;

use axum::{response::IntoResponse, Json};
use iris_client::IrisOrRequestError;
use reqwest::StatusCode;
use ris_client::{RisError, RisOrRequestError, RisUnauthorizedError};
use serde::{Deserialize, Serialize};

use utoipa::ToSchema;
use vendo_client::{VendoError, VendoOrRequestError};
use zugportal_client::error::ZugportalError;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RailboardApiError {
    pub domain: ErrorDomain,
    pub message: String,
    #[schema(nullable)]
    pub error: Option<UnderlyingApiError>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename = "lowercase")]
pub enum ErrorDomain {
    Vendo,
    Iris,
    Ris,
    Zugportal,
    Input,
    Request,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "errorType")]
pub enum UnderlyingApiError {
    #[serde(rename = "vendo")]
    Vendo(VendoError),
    #[serde(rename = "iris")]
    Iris,
    #[serde(rename = "ris-error")]
    RisError(RisError),
    #[serde(rename = "ris-unauthorized")]
    RisUnauthorizedError(RisUnauthorizedError),
    #[serde(rename = "zugportal-error")]
    ZugportalError(ZugportalError),
}

pub type RailboardResult<T> = Result<T, RailboardApiError>;

impl IntoResponse for RailboardApiError {
    fn into_response(self) -> axum::response::Response {
        let code = match self.domain {
            ErrorDomain::Vendo => StatusCode::BAD_REQUEST,
            ErrorDomain::Iris => StatusCode::BAD_REQUEST,
            ErrorDomain::Ris => StatusCode::BAD_REQUEST,
            ErrorDomain::Input => StatusCode::BAD_REQUEST,
            ErrorDomain::Zugportal => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorDomain::Request => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (code, Json(self)).into_response()
    }
}

impl From<ParseIntError> for RailboardApiError {
    fn from(value: ParseIntError) -> Self {
        RailboardApiError {
            domain: ErrorDomain::Input,
            message: format!("Required Integer but found: {value}"),
            error: None,
        }
    }
}

impl From<VendoOrRequestError> for RailboardApiError {
    fn from(value: VendoOrRequestError) -> Self {
        match value {
            VendoOrRequestError::FailedRequest(err) => RailboardApiError {
                domain: ErrorDomain::Request,
                message: format!("Failed to get from Vendo: {err}"),
                error: None,
            },
            VendoOrRequestError::VendoError(err) => RailboardApiError {
                domain: ErrorDomain::Vendo,
                message: format!("Failed to get from Vendo: {err}"),
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
                message: format!("Failed to get from Iris: {err}"),
                error: None,
            },
            IrisOrRequestError::IrisError(err) => RailboardApiError {
                domain: ErrorDomain::Iris,
                message: format!("Failed to get from Iris: {err}"),
                error: Some(UnderlyingApiError::Iris),
            },
            IrisOrRequestError::InvalidXML(err) => RailboardApiError {
                domain: ErrorDomain::Iris,
                message: format!("Got invalid/unrecognized xml from Iris: {err}"),
                error: None,
            },
        }
    }
}

impl From<RisOrRequestError> for RailboardApiError {
    fn from(value: RisOrRequestError) -> Self {
        match value {
            RisOrRequestError::FailedRequest(err) => RailboardApiError {
                domain: ErrorDomain::Request,
                message: format!("Failed to get from Ris: {err}"),
                error: None,
            },
            RisOrRequestError::RisError(err) => RailboardApiError {
                domain: ErrorDomain::Ris,
                message: format!("Failed to get from Ris: {err}"),
                error: Some(UnderlyingApiError::RisError(err)),
            },
            RisOrRequestError::RisUnauthorizedError(err) => RailboardApiError {
                domain: ErrorDomain::Ris,
                message: format!("The underlying request to ris was unauthorized: {err}"),
                error: Some(UnderlyingApiError::RisUnauthorizedError(err)),
            },
            RisOrRequestError::NotFoundError => RailboardApiError {
                domain: ErrorDomain::Input,
                message: "There was nothing found with these parameters".to_string(),
                error: None,
            }
        }
    }
}
