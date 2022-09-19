pub mod options;
mod util;

use crate::converter::options::*;
use pulldown_cmark::{html, Event, LinkType, Options, Parser, Tag};

pub struct Converter {
  options: ConverterOptions,
  pub unresolved_images: Vec<String>,
}

impl Converter {
  pub fn new(options: ConverterOptions) -> Self {
    Self {
      options,
      unresolved_images: vec![],
    }
  }

  pub fn parse(&mut self, markdown: &str) -> Result<(), String> {
    let mut options = Options::empty();
    let parser = Parser::new_ext(markdown, options);
    let parser = parser.map(|event| match &event {
      Event::Start(tag) => match &tag {
        // Store image url
        Tag::Image(LinkType::Inline, url, title) => {
          self.unresolved_images.push(url.to_string());
          Event::Start(Tag::Image(LinkType::Inline, title.clone(), url.clone()))
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

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert() {
    let mut options = ConverterOptions::new();
    options.heading_min.set(3);
    let mut converter = Converter::new(options);
    let markdown = "# Hello, world!\n\n![image_title](image_url)";
    converter.parse(markdown).unwrap();

    println!("{:?}", converter.unresolved_images);
  }
}
