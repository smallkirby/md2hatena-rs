use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Generate random alphanumeric string of size 32.
pub fn gen_uuid() -> String {
  thread_rng()
    .sample_iter(&Alphanumeric)
    .take(32)
    .map(char::from)
    .collect()
}

pub fn codename2extension(codename: &str) -> String {
  let parts = codename.split('.').collect::<Vec<_>>();
  if parts.len() == 1 {
    parts[0].into()
  } else {
    let extension = parts.last().unwrap();
    extension.to_string()
  }
}
