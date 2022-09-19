use hatena_rs::fotolife::Fotolife;
use hatena_rs::oauth::{consts::OauthScope, HatenaOauth};

/// Hatena Fotolife uploader
pub struct HatenaUploader {
  fotolife: Fotolife,
}

impl HatenaUploader {
  pub fn new() -> Self {
    let scopes = vec![
      OauthScope::WritePublic,
      OauthScope::WritePrivate,
      OauthScope::ReadPublic,
      OauthScope::ReadPrivate,
    ];
    let oauth = HatenaOauth::new(scopes, None).unwrap();
    let fotolife = Fotolife::new(oauth);

    HatenaUploader { fotolife }
  }
}
