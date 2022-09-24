use super::Codeblock;
use crate::util::codename2extension;

use pulldown_cmark::Event;

pub struct Highlightjs {}

impl Codeblock for Highlightjs {
  fn codeblock_start(&self, prog_name: &str) -> Vec<Event> {
    vec![
      Event::Html(r#"<pre>"#.into()),
      Event::Html(
        format!(
          r#"<code class="language-{}">"#,
          codename2extension(prog_name)
        )
        .into(),
      ),
    ]
  }

  fn codeblock_end(&self, _prog_name: &str) -> Vec<Event> {
    vec![
      Event::Html(r#"</code>"#.into()),
      Event::Html(r#"</pre>"#.into()),
    ]
  }

  fn predoc(&self) -> String {
    r#"
      <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/highlight.min.js"></script>
      <script src="//cdnjs.cloudflare.com/ajax/libs/highlightjs-line-numbers.js/2.8.0/highlightjs-line-numbers.min.js"></script>
      <script>hljs.highlightAll(); hljs.initLineNumbersOnLoad();</script>
      <!-- You have to add `<link href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/styles/default.min.css">` -->
    "#.into()
  }
}
