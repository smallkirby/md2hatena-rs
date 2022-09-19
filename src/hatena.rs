use crate::error::*;
use hatena_rs::fotolife::Fotolife;
use hatena_rs::oauth::{consts::OauthScope, HatenaOauth};

/// Hatena Fotolife uploader
pub struct HatenaUploader {
  fotolife: Fotolife,
  timeout: u64,
  myname: String,
}

impl HatenaUploader {
  /// Create new HatenaUploader
  ///
  /// While initialization, it fetches user information from Hatena OAuth API.
  ///
  /// # Arguments
  ///
  /// * `timeout` - Timeout in seconds for uploading images
  pub fn new(timeout: u64) -> Result<Self, ApplicationError> {
    let scopes = vec![
      OauthScope::WritePublic,
      OauthScope::WritePrivate,
      OauthScope::ReadPublic,
      OauthScope::ReadPrivate,
    ];
    let mut oauth = HatenaOauth::new(scopes, None).unwrap();
    let res = oauth.get_access_token(false)?;
    let fotolife = Fotolife::new(oauth);

    Ok(HatenaUploader {
      fotolife,
      timeout,
      myname: res.url_name,
    })
  }

  /// Upload image to Hatena Fotolife
  ///
  /// # Arguments
  ///
  /// * `path` - Path to image
  /// * `title - Title of image
  pub fn upload(&mut self, path: &std::path::Path, title: &str) -> Result<String, String> {
    let image = self.fotolife.post_image(path, title, self.timeout).unwrap();

    Ok(image.image_id)
  }

  pub fn fotolife_url(&self, image_id: &str) -> String {
    format!("https://f.hatena.ne.jp/{}/{}", self.myname, image_id)
  }
}
