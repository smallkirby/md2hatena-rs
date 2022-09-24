pub mod highlighjs;
pub mod pure;

use pulldown_cmark::Event;

pub trait Codeblock {
  fn codeblock_start(&self, prog_name: &str) -> Vec<Event>;
  fn codeblock_end(&self, prog_name: &str) -> Vec<Event>;
  fn predoc(&self) -> String;
}

impl dyn Codeblock {
  pub fn from(name: &str) -> Result<Box<dyn Codeblock>, ()> {
    match name {
      "highlightjs" | "highlight.js" => Ok(Box::new(highlighjs::Highlightjs {})),
      "pure" => Ok(Box::new(pure::Pure {})),
      _ => Err(()),
    }
  }
}
