use hatena_rs::oauth::error::OauthError;
use thiserror::Error;

use crate::hackmd::error::HackMDError;

#[derive(Debug, Error)]
pub enum ApplicationError {
  #[error("HackMD Error")]
  RequestFailure(#[from] HackMDError),

  #[error("HatenaOAuth Error")]
  JsonParseFailure(#[from] OauthError),

  #[error("authentication error: {message:?}")]
  AuthentiocationFailure { message: String },
}
