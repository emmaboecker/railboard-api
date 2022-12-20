use serde::Serialize;

use super::error::RailboardApiError;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T>
where
    T: Serialize,
{
    Success(T),
    Error(RailboardApiError),
}
