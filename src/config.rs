use serde::{Deserialize, Serialize};

use crate::{converter::options::HeadingDepth, error::ApplicationError};

/// Convert options for Markdown to Hatena HTML
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
  /// Minimum heading level
  /// eg: If 3, `#` heading is converted to `###`, `##` is to `####`
  #[serde(default)]
  pub heading_min: HeadingDepth,

  /// Directory to save temporary images
  #[serde(default = "default_download_dir")]
  pub download_dir: String,

  /// Timeout in seconds for uploading images
  #[serde(default = "default_timeout")]
  pub timeout: u64,
}

fn default_download_dir() -> String {
  "./.md2hatena-imgs".into()
}

fn default_timeout() -> u64 {
  10
}

impl Default for Config {
  fn default() -> Self {
    Config {
      heading_min: HeadingDepth::default(),
      download_dir: default_download_dir(),
      timeout: default_timeout(),
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
    let mut config: Config = if std::path::Path::new(&args.config_path).exists() {
      let config_str = std::fs::read_to_string(&args.config_path)?;
      serde_yaml::from_str(&config_str)?
    } else {
      Config::default()
    };

    if args.download_dir.is_some() {
      config.download_dir = args.download_dir.clone().unwrap();
    }
    if args.timeout.is_some() {
      config.timeout = args.timeout.unwrap();
    }

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
    ";
    let config: Config = serde_yaml::from_str(yml).unwrap();

    assert_eq!(
      config,
      Config {
        heading_min: HeadingDepth::new(3),
        timeout: 30,
        download_dir: default_download_dir(),
      }
    );
  }
}
