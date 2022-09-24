use pulldown_cmark::HeadingLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub struct HeadingDepth {
  depth: usize,
}

impl<'de> Deserialize<'de> for HeadingDepth {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let depth = s.parse::<usize>().unwrap_or(1);
    Ok(HeadingDepth { depth })
  }
}

impl Default for HeadingDepth {
  fn default() -> Self {
    HeadingDepth { depth: 1 }
  }
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
