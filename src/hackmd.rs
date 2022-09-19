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

  /// Get image from given `photo_url`.
  ///
  /// If `photo_url` starts with `https://hackmd.io/_uploads/`, this method fetch images
  /// from protected Amazon S3 server using HackMD's cookie.
  ///
  /// Otherwise, it just fetches the image without any cookies.
  ///
  /// # Arguments
  ///
  /// * `photo_url` - URL of the image
  pub fn get_photo(&self, photo_url: &str) -> Result<Bytes, HackMDError> {
    if photo_url.starts_with("https://hackmd.io/_uploads/") {
      let photo_name = photo_url.split('/').last().unwrap();
      self.get_protected_photo(photo_name)
    } else {
      self.get_normal_photo(photo_url)
    }
  }

  fn get_normal_photo(&self, photo_url: &str) -> Result<Bytes, HackMDError> {
    let client = Client::new();
    let res = client
      .get(photo_url)
      .header(USER_AGENT, "hackmd-rs")
      .send()?;

    match res.status() {
      StatusCode::OK => Ok(res.bytes()?),
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
  fn get_protected_photo(&self, photo_name: &str) -> Result<Bytes, HackMDError> {
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
