use serde::Deserialize;

use crate::{RisError, RisUnauthorizedError};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseOrRisError<Rsp> {
    Response(Box<Rsp>),
    Error(RisError),
    UnauthorizedError(RisUnauthorizedError),
}
