use pulldown_cmark::HeadingLevel;

pub struct HeadingDepth {
  depth: usize,
}

impl HeadingDepth {
  pub fn new(depth: usize) -> HeadingDepth {
    HeadingDepth { depth }
  }

  pub fn add(&self, num: usize) -> HeadingDepth {
    HeadingDepth::new(self.depth + num)
  }

  pub fn set(&mut self, num: usize) {
    self.depth = num;
  }

  pub fn to_level(&self) -> HeadingLevel {
    match self.depth {
      1 => HeadingLevel::H1,
      2 => HeadingLevel::H2,
      3 => HeadingLevel::H3,
      4 => HeadingLevel::H4,
      5 => HeadingLevel::H5,
      6 => HeadingLevel::H6,
      _ => HeadingLevel::H6,
    }
  }
}

/// Convert options for Markdown to Hatena HTML
pub struct ConverterOptions {
  /// Minimum heading level
  /// eg: If 3, `#` heading is converted to `###`, `##` is to `####`
  pub heading_min: HeadingDepth,
}

impl Default for ConverterOptions {
  fn default() -> Self {
    Self {
      heading_min: HeadingDepth::new(1),
    }
  }
}

impl ConverterOptions {
  pub fn new() -> ConverterOptions {
    ConverterOptions {
      heading_min: HeadingDepth::new(1),
    }
  }
}
