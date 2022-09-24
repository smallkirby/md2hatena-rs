use super::Codeblock;
use crate::util::codename2extension;

use pulldown_cmark::Event;

pub struct Highlightjs {}

impl Codeblock for Highlightjs {
  fn codeblock_start(&self, prog_name: &str) -> Vec<Event> {
    let extension = codename2extension(prog_name);
    let lang = if AVAILABLE_EXTENSIONS.contains(&extension.as_str()) {
      extension
    } else {
      "txt".into()
    };
    vec![
      // Add filename div
      Event::Html(r#"<div class="codeblock-title">"#.into()),
      Event::Text(prog_name.to_string().into()),
      Event::Html(r#"</div>"#.into()),
      // Add pre and class tag with appropriate lang class
      Event::Html(r#"<pre style="padding-top: 0; margin-top: 0;">"#.into()),
      Event::Html(format!(r#"<code class="language-{}">"#, lang,).into()),
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
      <script>hljs.highlightAll(); hljs.initLineNumbersOnLoad({singleLine:true});</script>
      <!-- You have to add `<link href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/styles/default.min.css">` -->
    "#.into()
  }
}

const AVAILABLE_EXTENSIONS: [&str; 370] = [
  "1c",
  "4d",
  "sap-abap",
  "abap",
  "abnf",
  "accesslog",
  "ada",
  "apex",
  "arduino",
  "ino",
  "armasm",
  "arm",
  "avrasm",
  "actionscript",
  "as",
  "alan",
  "i",
  "ln",
  "angelscript",
  "asc",
  "apache",
  "apacheconf",
  "applescript",
  "osascript",
  "arcade",
  "asciidoc",
  "adoc",
  "aspectj",
  "autohotkey",
  "autoit",
  "awk",
  "mawk",
  "nawk",
  "gawk",
  "bash",
  "sh",
  "zsh",
  "basic",
  "bbcode",
  "blade",
  "bnf",
  "brainfuck",
  "bf",
  "csharp",
  "cs",
  "c",
  "h",
  "cpp",
  "hpp",
  "cc",
  "hh",
  "c++",
  "h++",
  "cxx",
  "hxx",
  "cmake",
  "cmake.in",
  "cobol",
  "standard-cobol",
  "coq",
  "csp",
  "css",
  "capnproto",
  "capnp",
  "chaos",
  "kaos",
  "chapel",
  "chpl",
  "cisco",
  "clojure",
  "clj",
  "coffeescript",
  "coffee",
  "cson",
  "iced",
  "crystal",
  "cr",
  "curl",
  "cypher",
  "d",
  "dafny",
  "dart",
  "dpr",
  "dfm",
  "pas",
  "pascal",
  "diff",
  "patch",
  "django",
  "jinja",
  "dns",
  "zone",
  "bind",
  "dockerfile",
  "docker",
  "dos",
  "bat",
  "cmd",
  "dsconfig",
  "dts",
  "dust",
  "dst",
  "dylan",
  "ebnf",
  "elixir",
  "elm",
  "erlang",
  "erl",
  "excel",
  "xls",
  "xlsx",
  "extempore",
  "xtlang",
  "xtm",
  "fsharp",
  "fs",
  "fix",
  "fortran",
  "f90",
  "f95",
  "func",
  "gcode",
  "nc",
  "gams",
  "gms",
  "gauss",
  "gss",
  "godot",
  "gdscript",
  "gherkin",
  "hbs",
  "glimmer",
  "html.hbs",
  "html.handlebars",
  "htmlbars",
  "go",
  "golang",
  "golo",
  "gololang",
  "gradle",
  "graphql",
  "groovy",
  "gsql",
  "xml",
  "html",
  "xhtml",
  "rss",
  "atom",
  "xjb",
  "xsd",
  "xsl",
  "plist",
  "svg",
  "haskell",
  "hs",
  "haxe",
  "hx",
  "hlsl",
  "hy",
  "hylang",
  "ini",
  "toml",
  "inform7",
  "i7",
  "irpf90",
  "json",
  "java",
  "jsp",
  "javascript",
  "js",
  "jsx",
  "jolie",
  "iol",
  "ol",
  "julia",
  "julia-repl",
  "kotlin",
  "kt",
  "tex",
  "leaf",
  "lean",
  "lasso",
  "ls",
  "lassoscript",
  "less",
  "ldif",
  "lisp",
  "livecodeserver",
  "livescript",
  "ls",
  "lookml",
  "lua",
  "macaulay2",
  "makefile",
  "mk",
  "mak",
  "make",
  "markdown",
  "md",
  "mkdown",
  "mkd",
  "mathematica",
  "mma",
  "wl",
  "matlab",
  "maxima",
  "mel",
  "mercury",
  "mirc",
  "mrc",
  "mizar",
  "mkb",
  "mlir",
  "mojolicious",
  "monkey",
  "moonscript",
  "moon",
  "n1ql",
  "nsis",
  "never",
  "nginx",
  "nginxconf",
  "nim",
  "nimrod",
  "nix",
  "oak",
  "ocl",
  "ocaml",
  "ml",
  "objectivec",
  "mm",
  "objc",
  "obj-c",
  "obj-c++",
  "objective-c++",
  "ruleslanguage",
  "oxygene",
  "pf",
  "pf.conf",
  "php",
  "papyrus",
  "psc",
  "parser3",
  "perl",
  "pl",
  "pm",
  "pine",
  "pinescript",
  "plaintext",
  "txt",
  "text",
  "pony",
  "pgsql",
  "postgres",
  "postgresql",
  "powershell",
  "ps",
  "ps1",
  "processing",
  "prolog",
  "properties",
  "protobuf",
  "puppet",
  "pp",
  "python",
  "py",
  "gyp",
  "profile",
  "python-repl",
  "pycon",
  "qsharp",
  "k",
  "kdb",
  "qml",
  "r",
  "cshtml",
  "razor",
  "razor-cshtml",
  "reasonml",
  "re",
  "redbol",
  "rebol",
  "red",
  "red-system",
  "rib",
  "rsl",
  "risc",
  "riscript",
  "graph",
  "instances",
  "robot",
  "rf",
  "rpm-specfile",
  "rpm",
  "spec",
  "rpm-spec",
  "specfile",
  "scss",
  "sql",
  "p21",
  "step",
  "stp",
  "scala",
  "scheme",
  "scilab",
  "sci",
  "shexc",
  "shell",
  "console",
  "smali",
  "smalltalk",
  "st",
  "sml",
  "ml",
  "solidity",
  "sol",
  "spl",
  "stan",
  "stanfuncs",
  "stata",
  "iecst",
  "scl",
  "stl",
  "structured-text",
  "supercollider",
  "sc",
  "svelte",
  "swift",
  "tcl",
  "tk",
  "terraform",
  "tf",
  "hcl",
  "tap",
  "thrift",
  "toit",
  "tp",
  "tsql",
  "twig",
  "craftcms",
  "typescript",
  "ts",
  "unicorn-rails-log",
  "vbnet",
  "vb",
  "vba",
  "vbscript",
  "vbs",
  "vhdl",
  "vala",
  "verilog",
  "v",
  "vim",
  "xsharp",
  "xs",
  "prg",
  "axapta",
  "x++",
  "x86asm",
  "xl",
  "tao",
  "xquery",
  "xpath",
  "xq",
  "yml",
  "yaml",
  "zenscript",
  "zs",
  "zephir",
  "zep",
];
