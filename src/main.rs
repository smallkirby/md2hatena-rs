use std::env;

use hatena_rs::oauth::HatenaOauth;
use md2hatena::hackmd;

fn main() {
  let apitoken = env::var("HACKMD_APITOKEN").unwrap();
  let client = hackmd::HackMD::new(apitoken);
  let image = client.get_protected_photo("SJPb85Fgj.png").unwrap();
}
