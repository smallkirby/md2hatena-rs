pub mod image;
pub mod options;

use crate::config::Config;
use image::ResolvedImage;

use pulldown_cmark::{html, Event, LinkType, Options, Parser, Tag};

#[derive(Debug)]
struct ImageAltMapping {
  alt: String,
  url: String,
}

/// Converter of HackMD note to Hatena HTML
pub struct Converter {
  config: Config,
  pub unresolved_images: Vec<String>,
  resolved_images: Vec<ResolvedImage>,
  markdown: String,
  image_alt_mappings: Vec<ImageAltMapping>,
}

impl Converter {
  /// Create new converter
  ///
  /// # Arguments
  ///
  /// * `options` - Converter options
  pub fn new(config: &Config) -> Self {
    Self {
      config: config.clone(),
      unresolved_images: vec![],
      resolved_images: vec![],
      markdown: "".into(),
      image_alt_mappings: vec![],
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

    self.pre_parse(markdown);

    Ok(())
  }

  /// Convert HackMD note to Hatena HTML
  pub fn convert(&mut self) -> Result<String, String> {
    let markdown = self.markdown.clone();
    let html = self.convert_internal(&markdown)?;

    Ok(html)
  }

  /// Pre-parse Markdown content.
  ///
  /// - Check URL of unresolved images, then push to `Self.unresolved_images`
  /// - Check alt text of images, then push to `Self.image_alt_mappings`
  fn pre_parse(&mut self, markdown: &str) {
    let mut image_url: Option<String> = None;

    let parser = Parser::new_ext(markdown, Options::all()).map(|event| match &event {
      Event::Text(text) => {
        if image_url.is_some() {
          self.image_alt_mappings.push(ImageAltMapping {
            alt: text.to_string(),
            url: image_url.clone().unwrap().into(),
          });
          image_url = None;
        }
        event
      }

      Event::Start(Tag::Image(LinkType::Inline, url, _)) => {
        let resolved_image = self
          .resolved_images
          .iter()
          .find(|image| image.original_url == url.to_string());
        let unresolved_image = self
          .unresolved_images
          .iter()
          .find(|&image| image == &url.to_string());
        if unresolved_image.is_none() && resolved_image.is_none() {
          self.unresolved_images.push(url.to_string());
        }

        image_url = Some(url.to_string());
        event
      }

      _ => event,
    });

    let mut new_html = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut new_html, parser);
  }

  fn convert_internal(&mut self, markdown: &str) -> Result<String, String> {
    let mut in_image = false;

    let parser = Parser::new_ext(markdown, Options::all()).map(|event| match &event {
      Event::End(Tag::Image(LinkType::Inline, _, _)) => {
        in_image = false;
        vec![]
      }
      Event::Text(_) => {
        if in_image {
          vec![]
        } else {
          vec![event]
        }
      }
      // - Replace image URL
      // - Add <figcaption> tag if image has alt text
      Event::Start(tag) => match &tag {
        Tag::Image(LinkType::Inline, url, title) => {
          let resolved_image = self
            .resolved_images
            .iter()
            .find(|image| image.original_url == url.to_string());
          match resolved_image {
            Some(resolved_image) => {
              in_image = true;

              let alt_text = self
                .image_alt_mappings
                .iter()
                .find(|mapping| mapping.url == url.to_string())
                .map(|mapping| mapping.alt.clone())
                .unwrap_or_else(|| title.to_string());
              vec!(
                Event::Html(format!(
                  r#"<figure class="figure-image figure-image-fotolife mceNonEditable" title="{}">"#, alt_text).into()
                ),
                  Event::Html(format!(
                    r#"<img src="{}" alt="{}" class="hatena-fotolife" loading="lazy" itemprop="image" title="">"#,
                    resolved_image.fotolife_url, alt_text
                  ).into()),
                  Event::Html(r#"</img>"#.into()),
                  Event::Html(r#"<figcaption class="mceEditable">"#.into()),
                    Event::Text(alt_text.into()),
                  Event::Html(r#"</figcaption>"#.into()),
                Event::Html(r#"</figure>"#.into()),
              )
            }
            None => vec!(event),
          }
        }

        // Adjust heading level based on options
        Tag::Heading(level, fragment, classes) => {
          vec!(Event::Start(Tag::Heading(
          self.config.heading_min.add(*level as usize - 1).to_level(),
          *fragment,
          classes.clone(),
        )))},
        _ => vec!(event),
      },
      _ => vec!(event),
    }).flatten();

    let mut new_html = String::with_capacity(markdown.len() * 2);
    html::push_html(&mut new_html, parser);
    Ok(new_html)
  }

  /// Resolve image URL to Hatena Fotolife URL
  pub fn resolve_images(&mut self, resolved_images: &Vec<ResolvedImage>) {
    for image in resolved_images {
      let position = self
        .unresolved_images
        .iter()
        .position(|url| url == &image.original_url);
      if let Some(position) = position {
        self.unresolved_images.remove(position);
      }

      self.resolved_images.push(image.clone());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let mut options = Config::new();
    options.heading_min.set(3);
    let mut converter = Converter::new(&options);
    let markdown = "# Hello, world!\n\n![image_title](image_url)";
    converter.parse(markdown).unwrap();

    println!("{:?}", converter.unresolved_images);
  }
}
