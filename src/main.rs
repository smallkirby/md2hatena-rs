use std::{env, path};

use clap::Parser;
use md2hatena::{
  converter::{self, options::ConverterOptions, ResolvedImage},
  error::ApplicationError,
  hackmd, hatena, util,
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

  #[clap(short('t'), long("timeout"), value_parser, default_value = "10")]
  timeout: u64,
}

fn test(markdown: &str, download_dir: &path::Path, timeout: u64) -> Result<(), ApplicationError> {
  let apitoken = env::var("HACKMD_APITOKEN").unwrap();
  let client = hackmd::HackMD::new(apitoken);
  let mut fotolife = hatena::HatenaUploader::new(timeout)?;

  let mut converter = converter::Converter::new(ConverterOptions::new());
  converter.parse(markdown).unwrap();

  // Download images
  let unresolved_images = converter.unresolved_images.clone();
  for image in &unresolved_images {
    let save_path = download_dir.join(image.split('/').last().unwrap());
    println!("[+] Downloading {}...", image);
    let bytes = client.get_photo(&image).unwrap();
    std::fs::write(save_path, bytes).unwrap();
  }

  // Upload images
  let mut fotolife_ids = vec![];
  for image in &unresolved_images {
    let save_path = download_dir.join(image.split('/').last().unwrap());
    let uuid = util::gen_uuid();
    println!(
      "[+] Uploading {}...",
      save_path.to_string_lossy().to_string()
    );
    let uploaded_path = fotolife.upload(&save_path, &uuid).unwrap();
    println!("\t\tuploaded to {}", uploaded_path);
    fotolife_ids.push(fotolife.fotolife_url(&uploaded_path));
  }

  // Resolve images
  converter.resolve_images(ResolvedImage::from(unresolved_images, fotolife_ids));

  // Convert to HTML
  let html = converter.convert().unwrap();
  println!("{}", html);

  Ok(())
}

fn main() {
  let args = Args::parse();

  let timeout = args.timeout;
  let markdown_path = args.markdown_path;
  let download_dir = path::Path::new(&args.download_dir);
  if !download_dir.exists() {
    std::fs::create_dir(download_dir).unwrap();
  }

  if let Ok(markdown) = std::fs::read_to_string(&markdown_path) {
    test(&markdown, download_dir, timeout);
  } else {
    println!("[-] Failed to read markdown file: {}", markdown_path);
  }
}
