use crate::error::*;
use hatena_rs::fotolife::Fotolife;
use hatena_rs::oauth::{consts::OauthScope, HatenaConsumerInfo, HatenaOauth};

/// Hatena Fotolife uploader
pub struct HatenaUploader {
  fotolife: Fotolife,
  timeout: u64,
  myname: Option<String>,
}

impl HatenaUploader {
  /// Create new HatenaUploader
  ///
  /// While initialization, it fetches user information from Hatena OAuth API.
  ///
  /// # Arguments
  ///
  /// * `consumer_info` - Hatena consumer keys
  /// * `timeout` - Timeout in seconds for uploading images
  pub fn new(consumer_info: HatenaConsumerInfo, timeout: u64) -> Result<Self, ApplicationError> {
    let scopes = vec![
      OauthScope::WritePublic,
      OauthScope::WritePrivate,
      OauthScope::ReadPublic,
      OauthScope::ReadPrivate,
    ];
    let oauth = HatenaOauth::new(scopes, None, consumer_info).unwrap();
    let fotolife = Fotolife::new(oauth);

    Ok(HatenaUploader {
      fotolife,
      timeout,
      myname: None,
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

  pub fn fotolife_url(&mut self, image_id: &str, extension: &str) -> String {
    if self.myname.is_none() {
      self.init_profile();
    }
    let dir = &image_id[..8];
    format!(
      "https://cdn-ak.f.st-hatena.com/images/fotolife/s/{}/{}/{}.{}",
      self.myname.as_ref().unwrap(),
      dir,
      image_id,
      extension
    )
  }

  fn init_profile(&mut self) -> Result<(), ApplicationError> {
    let res = self.fotolife.oauth.get_access_token(false)?;
    self.myname = Some(res.url_name);

    Ok(())
  }
}
