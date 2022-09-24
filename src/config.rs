use serde::{Deserialize, Serialize};

use crate::{converter::options::HeadingDepth, error::ApplicationError};

use shellexpand::tilde;

/// Convert options for Markdown to Hatena HTML
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
  /// Minimum heading level
  /// eg: If 3, `#` heading is converted to `###`, `##` is to `####`
  #[serde(default)]
  pub heading_min: HeadingDepth,

  /// Directory to save temporary images
  #[serde(default = "default_download_dir")]
  pub download_dir: String,

  /// Path to cache file which stores mapping of image URL and Hatena Fotolife ID
  #[serde(default = "default_image_mapping")]
  pub image_mapping: String,

  /// Timeout in seconds for uploading images
  #[serde(default = "default_timeout")]
  pub timeout: u64,

  /// Output HTML file path
  #[serde(default = "default_output")]
  pub output: String,
}

fn default_download_dir() -> String {
  tilde(&"./.md2hatena-imgs").into()
}

fn default_output() -> String {
  tilde(&"").into()
}

fn default_timeout() -> u64 {
  10
}

fn default_image_mapping() -> String {
  "".into()
}

impl Default for Config {
  fn default() -> Self {
    Config {
      heading_min: HeadingDepth::default(),
      download_dir: default_download_dir(),
      timeout: default_timeout(),
      image_mapping: default_image_mapping(),
      output: default_output(),
    }
  }
}

impl Config {
  pub fn new() -> Self {
    Config::default()
  }

  /// Migrate command-line arguments to Config from config file
  ///
  /// # Arguments
  ///
  /// * `arg` - command-line arguments
  pub fn from_args(args: &crate::cli::Args) -> Result<Self, ApplicationError> {
    let config_path_str = &tilde(&args.config_path).to_string();
    let config_path = std::path::Path::new(config_path_str);
    let mut config: Config = if config_path.exists() {
      let config_str = std::fs::read_to_string(config_path)?;
      serde_yaml::from_str(&config_str)?
    } else {
      Config::default()
    };

    if args.download_dir.is_some() {
      config.download_dir = args.download_dir.clone().unwrap();
    }
    config.download_dir = tilde(&config.download_dir).into();

    if args.timeout.is_some() {
      config.timeout = args.timeout.unwrap();
    }

    if args.image_mapping.is_some() {
      config.image_mapping = args.image_mapping.clone().unwrap();
    }
    config.image_mapping = tilde(&config.image_mapping).into();

    if args.output.is_some() {
      config.output = args.output.clone().unwrap();
    } else {
      let tmp = std::path::Path::new(&args.markdown_path);
      config.output = tilde(&format!(
        "{}.{}",
        tmp.with_extension("").to_string_lossy().to_string(),
        "html"
      ))
      .into();
    }
    config.output = tilde(&config.output).into();

    Ok(config)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_yaml;

  #[test]
  fn test_parse_config() {
    let yml = "
      heading_min: 3
      timeout: 30
      download_dir: ~/.md2hatena-cache
      output: ~/test.html
    ";
    let config: Config = serde_yaml::from_str(yml).unwrap();

    assert_eq!(
      config,
      Config {
        heading_min: HeadingDepth::new(3),
        timeout: 30,
        download_dir: "~/.md2hatena-cache".into(),
        image_mapping: default_image_mapping(),
        output: "~/test.html".into(),
      }
    );
  }
}
