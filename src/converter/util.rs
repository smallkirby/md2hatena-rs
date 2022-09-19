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
