//! No personal value by its shape: no long numeric id, email address, phone number, token, IP address
//! (private or public), web address or host name, home path or account handle in the content or the
//! documents. The check goes by pattern alone: no list of names is kept in the repository.

use crate::support;

use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use support::{ROOT, content_files, files_under, read, shown};

/// The two values `docs/PERSONAL-DATA.md` allows: the owner's handle and address. Each is taken out of a
/// line before its shapes are read, and what stands around it is still checked.
const ALLOWED: [&str; 2] = ["dev@ahas.io", "ahas"];

/// Each shape a personal value takes, with the word a failure calls it by.
static SHAPES: LazyLock<Vec<(&str, Regex)>> = LazyLock::new(|| {
  [
    ("a long numeric id", r"\d{6,}"),
    ("an email address", r"[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)+"),
    (
      "a phone number",
      r"(?:\+\d{1,3}[\s.-]?)?(?:\(\d{2,4}\)[\s.-]?|\d{2,4}[\s.-])\d{3,4}[\s.-]?\d{4}",
    ),
    (
      "a token",
      r"(?i)\b(?:gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|xox[baprs]-[A-Za-z0-9-]{10,}|sk-[A-Za-z0-9_-]{16,}|AKIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{30,}|eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,})",
    ),
    ("a private or public IP address", r"\b\d{1,3}(?:\.\d{1,3}){3}\b"),
    ("a web address", r"(?i)\b(?:https?|ssh|ftp)://"),
    (
      "a host name",
      r"(?i)\b[a-z0-9-]+(?:\.[a-z0-9-]+)*\.(?:com|net|org|io|kr|dev|app|local|lan|internal|home)\b",
    ),
    ("a home path", r"(?:/home/|/Users/|~/)[A-Za-z0-9._-]+"),
    ("an account handle", r"(?:^|[^A-Za-z0-9._%+-])@[A-Za-z0-9_]{3,}"),
  ]
  .into_iter()
  .map(|(word, pattern)| (word, Regex::new(pattern).expect("a shape")))
  .collect()
});

/// Each line of `text` holding a personal value's shape, as `path:line: shape`.
fn hits(path: &str, text: &str) -> Vec<String> {
  let mut found = Vec::new();
  for (number, line) in text.lines().enumerate() {
    let line = ALLOWED.iter().fold(line.to_owned(), |line, allowed| line.replace(allowed, ""));
    for (word, shape) in SHAPES.iter() {
      if shape.is_match(&line) {
        found.push(format!("{path}:{}: {word}", number + 1));
      }
    }
  }
  found
}

/// The content, the documents and the root's markdown.
fn read_files() -> Vec<String> {
  let mut paths: Vec<String> = content_files().iter().map(|path| shown(path)).collect();
  paths.extend(files_under(&Path::new(ROOT).join("docs")).iter().map(|path| shown(path)));
  for name in ["README.md", "CLAUDE.md"] {
    if Path::new(ROOT).join(name).is_file() {
      paths.push(name.to_owned());
    }
  }
  paths
}

#[test]
fn no_personal_value_by_its_shape() {
  let found: Vec<String> = read_files().iter().flat_map(|path| hits(path, &read(path))).collect();
  assert!(found.is_empty(), "these lines hold a personal value's shape: {found:?}");
}

#[test]
fn each_shape_is_found() {
  let sample = "\
id 1234567
write to someone@example.com
call 010-1234-5678
token ghp_ABCDEFGHIJKLMNOPQRSTUVWXyz
at 10.0.0.12
at 203.0.113.7
see https://example.invalid
on host.example.net
under /home/someone
ask @some_bot
";
  assert_eq!(
    hits("sample", sample),
    [
      "sample:1: a long numeric id",
      "sample:2: an email address",
      "sample:2: a host name",
      "sample:3: a phone number",
      "sample:4: a token",
      "sample:5: a private or public IP address",
      "sample:6: a private or public IP address",
      "sample:7: a web address",
      "sample:8: a host name",
      "sample:9: a home path",
      "sample:10: an account handle",
    ]
  );
  assert!(hits("sample", "read `docs/lore/index.md`, id {{owner.id}}, a 12345 count").is_empty());
  assert!(hits("sample", "ask ahas, write to dev@ahas.io").is_empty());
}
