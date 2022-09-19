pub mod consts;
pub mod cookie;
pub mod error;

use bytes::Bytes;
use reqwest::{
  blocking::Client,
  header::{AUTHORIZATION, COOKIE, USER_AGENT},
  StatusCode,
};
use serde::{Deserialize, Serialize};

use self::{consts::Cookie, cookie::HackMDCookie, error::HackMDError};

/// HackMD client
pub struct HackMD {
  api_token: String,    // API token to access HackMD
  cookie: HackMDCookie, // Cookie manager
}

/// User information
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
  pub id: String,
  pub name: String,
  pub email: String,
  pub user_path: String,
  pub photo: String,
  pub teams: Vec<TeamInfo>,
}

/// Team information
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamInfo {
  pub id: String,
  pub owner_id: String,
  pub path: String,
  pub name: String,
  pub logo: String,
  pub description: String,
  pub visibility: String,
  pub created_at: Option<u64>,
}

impl HackMD {
  /// Create a new HackMD client
  pub fn new(api_token: String) -> Self {
    let cookie_env = std::env::var(Cookie::ENV_HACKMD_COOKIE).unwrap_or("".into());
    let cookie = HackMDCookie::new(if cookie_env.is_empty() {
      None
    } else {
      Some(cookie_env)
    });

    Self { api_token, cookie }
  }

  /// Get user information of me
  pub fn me(&self) -> Result<UserInfo, HackMDError> {
    let client = Client::new();
    let res = client
      .get("https://api.hackmd.io/v1/me")
      .header(USER_AGENT, "hackmd-rs")
      .header(AUTHORIZATION, format!("Bearer {}", self.api_token))
      .send()?;

    match res.status() {
      StatusCode::OK => {
        let me: UserInfo = serde_json::from_str(&res.text()?)?;
        Ok(me)
      }
      StatusCode::UNAUTHORIZED => Err(HackMDError::AuthentiocationFailure {
        message: "API token is invalid?".into(),
      }),
      _ => Err(HackMDError::RequestFailure(
        res.error_for_status().unwrap_err(),
      )),
    }
  }

  /// Get protected image from HackMD, which redirects to S3 bucket storage
  ///
  /// # Arguments
  ///
  /// * `photo_name` - URL of protected image
  pub fn get_protected_photo(&self, photo_name: &str) -> Result<Bytes, HackMDError> {
    let cookie = self.cookie.get_cookie(false)?;
    let client = Client::new();
    let res = client
      .get(format!("https://hackmd.io/_uploads/{}", photo_name))
      .header(USER_AGENT, "hackmd-rs")
      .header(AUTHORIZATION, format!("Bearer {}", self.api_token))
      .header(COOKIE, cookie)
      .send()?;

    match res.status() {
      StatusCode::OK => Ok(res.bytes()?),
      StatusCode::UNAUTHORIZED => Err(HackMDError::AuthentiocationFailure {
        message: "Cookie is invalid?".into(),
      }),
      _ => Err(HackMDError::RequestFailure(
        res.error_for_status().unwrap_err(),
      )),
    }
  }
}
