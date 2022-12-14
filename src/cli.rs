use std::{env, fs, io::Write, path, process::exit};

use crate::{
  converter::image::ResolvedImage, error::ApplicationError, hackmd::HackMD, hatena::HatenaUploader,
  util,
};

use clap::Parser;
use colored::*;
use hatena_rs::oauth::HatenaConsumerInfo;
use indicatif::ProgressBar;

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  /// Path to Markdown file to convert
  #[clap(value_parser)]
  pub markdown_path: String,

  /// Directory to save temporary images
  #[clap(short('d'), long("download-dir"), value_parser)]
  pub download_dir: Option<String>,

  /// Timeout in seconds for uploading images
  #[clap(short('t'), long("timeout"), value_parser)]
  pub timeout: Option<u64>,

  /// Don't upload images to Hatena Fotolife, and don't resolve image URL
  #[clap(short('n'), long("no-resolve"), value_parser, default_value = "false")]
  pub no_resolve: bool,

  /// Path to cache file which stores mapping of image URL and Hatena Fotolife ID
  #[clap(short('i'), long("image-cache"), value_parser)]
  pub image_mapping: Option<String>,

  /// Path to output HTML file
  #[clap(short('o'), long("output"), value_parser)]
  pub output: Option<String>,

  /// Path to configuration file
  #[clap(
    short('c'),
    long("config"),
    value_parser,
    default_value = "~/.md2hatena.config.yml"
  )]
  pub config_path: String,
}

/// Exit with error message
pub fn panic_with_error(err: ApplicationError) {
  match err {
    ApplicationError::RequestFailure(e) => {
      eprintln!("{} {}", "[!] Error:".red().bold(), e);
    }
    ApplicationError::OAuthFailure(e) => {
      eprintln!("{} {}", "[!] Error:".red().bold(), e);
    }
    ApplicationError::AuthentiocationFailure { message } => {
      eprintln!("{} {}", "[!] Error:".red().bold(), message);
    }
    ApplicationError::FileIoFailure(e) => {
      eprintln!("{} {}", "[!] Error:".red().bold(), e);
    }
    ApplicationError::ConfigParseFailure(e) => {
      eprintln!("{} {}", "[!] Error:".red().bold(), e);
    }
    ApplicationError::MiscError { message } => {
      eprintln!("{} {}", "[!] Error:".red().bold(), message);
    }
  }
  exit(1);
}

/// Check necessary API tokens of Hatena in envvar, and returns them
///
/// Note that this function panics if necessary tokens not found.
pub fn get_hatena_api_token() -> HatenaConsumerInfo {
  if env::var("HATENA_CONSUMER_KEY").is_err() {
    println!(
      "{} {} is not set as envvar.",
      "[!]".red().bold(),
      "HATENA_CONSUMER_KEY".bright_green()
    );
    exit(1);
  }
  if env::var("HATENA_CONSUMER_SECRET").is_err() {
    println!(
      "{} {} is not set as envvar.",
      "[!]".red().bold(),
      "HATENA_CONSUMER_SECRET".bright_green()
    );
    exit(1);
  }

  HatenaConsumerInfo::new(
    &env::var("HATENA_CONSUMER_KEY").unwrap(),
    &env::var("HATENA_CONSUMER_SECRET").unwrap(),
  )
  .unwrap()
}

/// Check necessary API tokens of HackMD in envvar, and returns HackMD API token
///
/// Note that this function panics if necessary tokens not found.
pub fn get_hackmd_api_token() -> String {
  if env::var("HACKMD_APITOKEN").is_err() {
    println!(
      "{} {} is not set as envvar.",
      "[!]".red().bold(),
      "HACKMD_APITOKEN".bright_green()
    );
    exit(1);
  }

  env::var("HACKMD_APITOKEN").unwrap()
}

/// Read markdown file and returns its content
///
/// Note that this function panics if file not found.
pub fn read_markdown_file(path: &str) -> String {
  if let Ok(markdown) = std::fs::read_to_string(&path) {
    markdown
  } else {
    println!(
      "{} Failed to read markdown file: {}",
      "[!]".red().bold(),
      path
    );
    exit(1)
  }
}

/// Download images from Network with progress bar
pub fn download_images(
  images: &[String],
  download_dir: &path::Path,
  hackmd_client: &HackMD,
  use_cache: bool,
) {
  if images.is_empty() {
    return;
  }

  let images: Vec<String> = if use_cache {
    images
      .iter()
      .filter(|image| {
        let image_path = download_dir.join(image.split('/').last().unwrap());
        !image_path.exists()
      })
      .map(|image| image.to_string())
      .collect()
  } else {
    images.to_vec()
  };

  println!("{} Downloading images from HackMD", "[+]".green().bold(),);
  let pb = ProgressBar::new(images.len() as u64);
  pb.set_style(
    indicatif::ProgressStyle::with_template(
      "  {spinner:.green} [{pos}/{len}] [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}",
    )
    .unwrap()
    .progress_chars("#>-"),
  );

  for image in images {
    pb.set_message(image.clone());
    std::thread::sleep(std::time::Duration::from_millis(500));
    let save_path = download_dir.join(image.split('/').last().unwrap());
    let bytes = hackmd_client.get_photo(&image).unwrap();
    fs::write(save_path, bytes).unwrap();
    pb.inc(1);
  }

  pb.finish_with_message("Done");
}

/// Upload images to Hatena Fotolife
pub fn upload_images(
  #[allow(clippy::ptr_arg)] images: &Vec<String>,
  download_dir: &path::Path,
  hatena: &mut HatenaUploader,
  use_cache: bool,
  cache_path: &str,
) -> Result<Vec<String>, ApplicationError> {
  if images.is_empty() {
    return Ok(vec![]);
  }
  let mut fotolife_ids = vec![];
  hatena.init_profile().unwrap();

  let images = if use_cache {
    images
      .iter()
      .filter(|image| {
        let image_path = download_dir.join(image.split('/').last().unwrap());
        !image_path.exists()
      })
      .map(|image| image.to_string())
      .collect()
  } else {
    images.clone()
  };

  println!(
    "{} Uploading images to Hatena Fotolife",
    "[+]".green().bold(),
  );
  let pb = ProgressBar::new(images.len() as u64);
  pb.set_style(
    indicatif::ProgressStyle::with_template(
      "  {spinner:.green} [{pos}/{len}] [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}",
    )
    .unwrap()
    .progress_chars("#>-"),
  );

  for image in images {
    let save_path = download_dir.join(image.split('/').last().unwrap());
    let extension = save_path.extension().unwrap().to_str().unwrap();
    pb.set_message(save_path.to_string_lossy().to_string());
    // Download image
    let uuid = util::gen_uuid();
    let uploaded_path = hatena.upload(&save_path, &uuid).unwrap();
    fotolife_ids.push(hatena.fotolife_url(&uploaded_path, extension));
    // Cache image mapping
    if !cache_path.is_empty() {
      let resolved_image = ResolvedImage::from(
        vec![image.to_string()],
        vec![hatena.fotolife_url(&uploaded_path, extension)],
      );
      ResolvedImage::cache_to(&resolved_image, cache_path)?;
    }

    pb.inc(1);
  }

  pb.finish_with_message("Done");
  Ok(fotolife_ids)
}

pub fn write_result_html(html: &str, output_path: &str) {
  let output_path = path::Path::new(output_path);
  let output_dir = output_path.parent().unwrap();
  if !output_dir.exists() {
    fs::create_dir_all(output_dir).unwrap();
  }

  let mut file = fs::File::create(output_path).unwrap();
  file.write(html.as_bytes()).unwrap();

  println!(
    "{} Output HTML: {}",
    "[+]".green().bold(),
    output_path.display()
  );
}
