use std::path;

use clap::Parser;
use md2hatena::{
  cli::{download_images, get_api_token, panic_with_error, read_markdown_file, upload_images},
  converter::{self, options::ConverterOptions, ResolvedImage},
  error::ApplicationError,
  hackmd, hatena,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to Markdown file to convert
  #[clap(value_parser)]
  markdown_path: String,

  /// Directory to save temporary images
  #[clap(
    short('d'),
    long("download-dir"),
    value_parser,
    default_value = "./.md2hatena-imgs"
  )]
  download_dir: String,

  /// Timeout in seconds for uploading images
  #[clap(short('t'), long("timeout"), value_parser, default_value = "10")]
  timeout: u64,

  /// Don't use cached images
  #[clap(short('n'), long("no-cache"), value_parser, default_value = "false")]
  no_cache: bool,
}

fn process() -> Result<(), ApplicationError> {
  let args = Args::parse();
  let timeout = args.timeout;
  let markdown_path = args.markdown_path;
  let download_dir = path::Path::new(&args.download_dir);
  if !download_dir.exists() {
    std::fs::create_dir(download_dir).unwrap();
  }
  let no_cache = args.no_cache;

  let hackmd_apitoken = get_api_token();
  let markdown = read_markdown_file(&markdown_path);

  let hackmd = hackmd::HackMD::new(hackmd_apitoken);
  let mut fotolife = hatena::HatenaUploader::new(timeout)?;

  let mut converter = converter::Converter::new(ConverterOptions::new());
  converter.parse(&markdown).unwrap();

  // Download images
  let unresolved_images = converter.unresolved_images.clone();
  download_images(&unresolved_images, download_dir, &hackmd, !no_cache);

  // Upload images
  let fotolife_ids = upload_images(&unresolved_images, download_dir, &mut fotolife, !no_cache);

  // Resolve images
  converter.resolve_images(ResolvedImage::from(unresolved_images, fotolife_ids));

  // Convert to HTML
  let html = converter.convert().unwrap();

  println!("{}", html);

  Ok(())
}

fn main() {
  match process() {
    Ok(()) => (),
    Err(err) => panic_with_error(err),
  }
}
