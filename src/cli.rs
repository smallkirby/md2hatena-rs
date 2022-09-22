use std::{env, fs, path, process::exit};

use crate::{error::ApplicationError, hackmd::HackMD, hatena::HatenaUploader, util};

use colored::*;
use indicatif::ProgressBar;

/// Exit with error message
pub fn panic_with_error(err: ApplicationError) {
  match err {
    ApplicationError::RequestFailure(e) => {
      eprintln!("{} {}", "[!] Error:".red().bold(), e);
    }
    ApplicationError::JsonParseFailure(e) => {
      eprintln!("{} {}", "[!] Error:".red().bold(), e);
    }
    ApplicationError::AuthentiocationFailure { message } => {
      eprintln!("{} {}", "[!] Error:".red().bold(), message);
    }
  }
  exit(1);
}

/// Check necessary API tokens in envvar, and returns HackMD API token
///
/// Note that this function panics if necessary tokens not found.
pub fn get_api_token() -> String {
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
pub fn download_images(images: &Vec<String>, download_dir: &path::Path, hackmd_client: &HackMD) {
  if images.is_empty() {
    return;
  }

  println!(
    "{} {}",
    "[+]".green().bold(),
    "Downloading images from HackMD"
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
  images: &Vec<String>,
  download_dir: &path::Path,
  hatena: &mut HatenaUploader,
) -> Vec<String> {
  if images.is_empty() {
    return vec![];
  }
  let mut fotolife_ids = vec![];

  println!(
    "{} {}",
    "[+]".green().bold(),
    "Uploading images to Hatena Fotolife"
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
    pb.set_message(save_path.to_string_lossy().to_string());
    let uuid = util::gen_uuid();
    let uploaded_path = hatena.upload(&save_path, &uuid).unwrap();
    fotolife_ids.push(hatena.fotolife_url(&uploaded_path));

    pb.inc(1);
  }

  pb.finish_with_message("Done");
  fotolife_ids
}
