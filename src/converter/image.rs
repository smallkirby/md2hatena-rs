use std::io::prelude::*;

use crate::error::ApplicationError;

#[derive(Debug, PartialEq, Clone)]
pub struct ResolvedImage {
  pub original_url: String,
  pub fotolife_url: String,
}

impl ResolvedImage {
  pub fn from(original_urls: Vec<String>, fotolife_ids: Vec<String>) -> Vec<Self> {
    if original_urls.len() != fotolife_ids.len() {
      return vec![];
    }

    original_urls
      .iter()
      .zip(fotolife_ids.iter())
      .map(|(original_url, fotolife_id)| ResolvedImage {
        original_url: original_url.to_string(),
        fotolife_url: fotolife_id.to_string(),
      })
      .collect()
  }

  pub fn cache_to(images: &Vec<Self>, cache_path: &str) -> Result<(), ApplicationError> {
    let cache_path = std::path::Path::new(cache_path);

    // Remove duplicating images
    let existing_images = if cache_path.exists() {
      Self::restore_from(&cache_path.to_string_lossy().to_string())?
    } else {
      vec![]
    };
    let images = images
      .iter()
      .filter(|image| !existing_images.contains(image))
      .collect::<Vec<_>>();

    // Convert to string
    let cache_data: Vec<String> = images
      .iter()
      .map(|image| format!("{} -> {}", image.original_url, image.fotolife_url,))
      .collect();

    // Write to cache file
    let mut file = std::fs::OpenOptions::new()
      .write(true)
      .append(true)
      .create(true)
      .open(cache_path)?;
    for data in cache_data {
      writeln!(file, "{}", data)?;
    }

    Ok(())
  }

  pub fn restore_from(cache_path: &str) -> Result<Vec<ResolvedImage>, ApplicationError> {
    let cache_path = std::path::Path::new(cache_path);
    let contents = if let Ok(contents) = std::fs::read_to_string(cache_path) {
      contents
    } else {
      return Ok(vec![]);
    };

    let mut images: Vec<ResolvedImage> = vec![];
    for line in contents.lines() {
      let mut iter = line.split(" -> ");
      let original_url = iter.next().unwrap().to_string();
      let fotolife_url = iter.next().unwrap().to_string();
      images.push(ResolvedImage {
        original_url,
        fotolife_url,
      });
    }

    Ok(images)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn trim_indent(s: &str) -> String {
    let mut lines = vec![];
    for line in s.lines() {
      if line.trim().is_empty() {
        continue;
      }
      lines.push(line.trim());
    }

    lines.join("\n")
  }

  #[test]
  fn test_cache_images() {
    let resolved_images = vec![
      ResolvedImage {
        original_url: "https://example.com/image1.png".into(),
        fotolife_url: "https://f.hatena.ne.jp/username/20200101/1234567890.png".into(),
      },
      ResolvedImage {
        original_url: "https://example.com/image2.png".into(),
        fotolife_url: "https://f.hatena.ne.jp/username/20200101/1234567891.png".into(),
      },
    ];
    let cache_path = ".cache";

    ResolvedImage::cache_to(&resolved_images, cache_path).unwrap();

    let cache_content = std::fs::read_to_string(cache_path).unwrap();
    assert_eq!(
      cache_content.trim(),
      trim_indent(
        "
        https://example.com/image1.png -> https://f.hatena.ne.jp/username/20200101/1234567890.png
        https://example.com/image2.png -> https://f.hatena.ne.jp/username/20200101/1234567891.png
      "
      )
    );
  }
}
