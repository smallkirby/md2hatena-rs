use std::path;

use clap::Parser;
use md2hatena::{
  cli::{
    download_images, get_hackmd_api_token, get_hatena_api_token, panic_with_error,
    read_markdown_file, upload_images, write_result_html, Args,
  },
  config::Config,
  converter::{self, image::ResolvedImage},
  error::ApplicationError,
  hackmd, hatena,
};

fn process() -> Result<(), ApplicationError> {
  let args = Args::parse();
  let config = Config::from_args(&args)?;

  let hackmd_apitoken = get_hackmd_api_token();
  let hatena_apitoken = get_hatena_api_token();
  let markdown = read_markdown_file(&args.markdown_path);

  let hackmd = hackmd::HackMD::new(hackmd_apitoken);
  let mut fotolife = hatena::HatenaUploader::new(hatena_apitoken, config.timeout)?;

  let mut converter = converter::Converter::new(&config);
  converter.parse(&markdown).unwrap();

  let do_image_cache = !config.image_mapping.is_empty();
  if !args.no_resolve {
    // Resolve images using cached ones first
    let cached_images = if do_image_cache {
      ResolvedImage::restore_from(&config.image_mapping)?
    } else {
      vec![]
    };
    converter.resolve_images(&cached_images);

    // Download images
    let unresolved_images = converter.unresolved_images.clone();
    download_images(
      &unresolved_images,
      path::Path::new(&config.download_dir),
      &hackmd,
      false,
    );

    // Upload images
    let fotolife_ids = upload_images(
      &unresolved_images,
      path::Path::new(&config.download_dir),
      &mut fotolife,
      false,
    );

    // Resolve images
    let resolved_images = ResolvedImage::from(unresolved_images, fotolife_ids);
    converter.resolve_images(&resolved_images);
    if do_image_cache {
      ResolvedImage::cache_to(&resolved_images, &config.image_mapping)?;
    }
  }

  // Convert to HTML
  let html = converter.convert().unwrap();
  write_result_html(&html, &config.output);

  Ok(())
}

fn main() {
  match process() {
    Ok(()) => (),
    Err(err) => panic_with_error(err),
  }
}
