pub mod options;

use crate::converter::options::*;

use pulldown_cmark::{html, Event, LinkType, Options, Parser, Tag};

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
}

/// Converter of HackMD note to Hatena HTML
pub struct Converter {
  options: ConverterOptions,
  pub unresolved_images: Vec<String>,
  resolved_images: Vec<ResolvedImage>,
  markdown: String,
}

impl Converter {
  /// Create new converter
  ///
  /// # Arguments
  ///
  /// * `options` - Converter options
  pub fn new(options: ConverterOptions) -> Self {
    Self {
      options,
      unresolved_images: vec![],
      resolved_images: vec![],
      markdown: "".into(),
    }
  }

  /// Parse HackMD note and apply fixes on markdown
  ///
  /// # Arguments
  ///
  /// * `markdown` - HackMD note
  pub fn parse(&mut self, markdown: &str) -> Result<(), String> {
    self.markdown = markdown.into();
    self.resolved_images.clear();
    self.unresolved_images.clear();

    let _ = self.convert_internal(markdown, true);

    Ok(())
  }

  /// Convert HackMD note to Hatena HTML
  pub fn convert(&mut self) -> Result<String, String> {
    let markdown = self.markdown.clone();
    let html = self.convert_internal(&markdown, false)?;

    Ok(html)
  }

  fn convert_internal(&mut self, markdown: &str, do_parse: bool) -> Result<String, String> {
    let parser = Parser::new(markdown).map(|event| match &event {
      Event::Start(tag) => match &tag {
        // Store image url
        Tag::Image(LinkType::Inline, url, title) => {
          if do_parse {
            self.unresolved_images.push(url.to_string());
            Event::Start(Tag::Image(LinkType::Inline, title.clone(), url.clone()))
          } else {
            let resolved_image = self
              .resolved_images
              .iter()
              .find(|image| image.original_url == url.to_string());
            match resolved_image {
              Some(resolved_image) => Event::Start(Tag::Image(
                LinkType::Inline,
                resolved_image.fotolife_url.clone().into(),
                title.clone(),
              )),
              None => Event::Start(Tag::Image(LinkType::Inline, title.clone(), url.clone())),
            }
          }
        }
        // Adjust heading level based on options
        Tag::Heading(level, fragment, classes) => Event::Start(Tag::Heading(
          self.options.heading_min.add(*level as usize - 1).to_level(),
          fragment.clone(),
          classes.clone(),
        )),
        _ => event,
      },
      _ => event,
    });

    // Execute parse
    let mut html = String::new();
    html::push_html(&mut html, parser);

    Ok(html)
  }

  /// Resolve image URL to Hatena Fotolife URL
  pub fn resolve_images(&mut self, resolved_images: Vec<ResolvedImage>) {
    for image in resolved_images {
      self.unresolved_images.remove(
        self
          .unresolved_images
          .iter()
          .position(|url| url == &image.original_url)
          .unwrap(),
      );
      self.resolved_images.push(image);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let mut options = ConverterOptions::new();
    options.heading_min.set(3);
    let mut converter = Converter::new(options);
    let markdown = "# Hello, world!\n\n![image_title](image_url)";
    converter.parse(markdown).unwrap();

    println!("{:?}", converter.unresolved_images);
  }
}
