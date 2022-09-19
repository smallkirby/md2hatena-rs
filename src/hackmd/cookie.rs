use std::io::{self, Write};

use crate::hackmd::error::HackMDError;

/// HackMD client to get a logged-in cookie.
///
/// This struct needs user interaction to retrieve cookie.
pub struct HackMDCookie {
  cookie: Option<String>,
}

impl HackMDCookie {
  /// Create a new HackMD cookie client
  ///
  /// # Arguments
  ///
  /// * `default_cookie` - Pre-fetched cookie from envvar
  pub fn new(default_cookie: Option<String>) -> Self {
    let cookie = if let Some(cookie) = default_cookie {
      if cookie.starts_with("connect.sid") {
        Some(cookie)
      } else {
        Some(format!("connect.sid={}", cookie))
      }
    } else {
      None
    };

    Self { cookie }
  }

  /// Get a logged-in cookie.
  ///
  /// This functions opens a browser and asks user to login.
  ///
  /// # Arguments
  ///
  /// * `dont_use_cache` - If true, this function ignores the cached cookie.
  pub fn get_cookie(&self, dont_use_cache: bool) -> Result<String, HackMDError> {
    if !dont_use_cache && self.cookie.is_some() {
      return Ok(self.cookie.clone().unwrap());
    }

    let mut cookie = String::new();
    webbrowser::open("https://hackmd.io").map_err(|_| HackMDError::AuthentiocationFailure {
      message: "User rejects to login".into(),
    })?;

    print!("Input 'connect.sid' cookie found in a browser's devtool: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cookie).unwrap();

    if !cookie.starts_with("connect.sid") {
      cookie = format!("connect.sid={}", cookie);
    }

    Ok(cookie)
  }
}
