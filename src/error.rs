use hatena_rs::oauth::error::OauthError;
use thiserror::Error;

use crate::hackmd::error::HackMDError;

#[derive(Debug, Error)]
pub enum ApplicationError {
  #[error("HackMD Error")]
  RequestFailure(#[from] HackMDError),

  #[error("HatenaOAuth Error")]
  OAuthFailure(#[from] OauthError),

  #[error("authentication error: {message:?}")]
  AuthentiocationFailure { message: String },

  #[error("File IO Failure")]
  FileIoFailure(#[from] std::io::Error),

  #[error("Config parse failure")]
  ConfigParseFailure(#[from] serde_yaml::Error),

  #[error("Misc error: {message:?}")]
  MiscError { message: String },
}
