use pulldown_cmark::{CodeBlockKind, Event, Tag};

use super::Codeblock;

pub struct Pure {}

impl Codeblock for Pure {
  fn codeblock_start(&self, prog_name: &str) -> Vec<Event> {
    vec![Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(
      prog_name.to_string().into(),
    )))]
  }

  fn codeblock_end(&self, prog_name: &str) -> Vec<Event> {
    vec![Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(
      prog_name.to_string().into(),
    )))]
  }

  fn predoc(&self) -> String {
    "".into()
  }
}
