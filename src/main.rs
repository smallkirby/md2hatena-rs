use std::env;
use std::path;

use clap::Parser;
use md2hatena::{
  converter::{self, options::ConverterOptions},
  hackmd,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(value_parser)]
  markdown_path: String,

  #[clap(
    short('d'),
    long("download-dir"),
    value_parser,
    default_value = "./md2hatena-imgs"
  )]
  download_dir: String,
}

fn test(markdown: &str, download_dir: &path::Path) {
  let apitoken = env::var("HACKMD_APITOKEN").unwrap();
  let client = hackmd::HackMD::new(apitoken);

  let mut converter = converter::Converter::new(ConverterOptions::new());
  converter.parse(markdown).unwrap();

  let unresolved_images = converter.unresolved_images;
  for image in unresolved_images {
    let save_path = download_dir.join(image.split('/').last().unwrap());
    println!("[+] Downloading {}...", image);
    let bytes = client.get_photo(&image).unwrap();
    std::fs::write(save_path, bytes).unwrap();
  }
}

fn main() {
  let args = Args::parse();

  let markdown_path = args.markdown_path;
  let download_dir = path::Path::new(&args.download_dir);
  if !download_dir.exists() {
    std::fs::create_dir(download_dir).unwrap();
  }

  if let Ok(markdown) = std::fs::read_to_string(&markdown_path) {
    test(&markdown, download_dir);
  } else {
    println!("[-] Failed to read markdown file: {}", markdown_path);
  }
}
