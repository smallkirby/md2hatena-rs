use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HackMDError {
  #[error("request failed")]
  RequestFailure(#[from] reqwest::Error),

  #[error("json parse failed")]
  JsonParseFailure(#[from] SerdeError),

  #[error("authentication error: {message:?}")]
  AuthentiocationFailure { message: String },
}
