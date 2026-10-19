//! No personal value by its shape: no long numeric id, email address, IP address, web address or host
//! name, home path or account handle in the content or the documents. The check goes by pattern alone:
//! no list of names is kept in the repository.

mod support;

use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use support::{ROOT, content_files, files_under, read, shown};

/// Each shape a personal value takes, with the word a failure calls it by.
static SHAPES: LazyLock<Vec<(&str, Regex)>> = LazyLock::new(|| {
  [
    ("a long numeric id", r"\d{6,}"),
    ("an email address", r"[A-Za-z0-9._%+-]+@[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)+"),
    ("an IP address", r"\b\d{1,3}(?:\.\d{1,3}){3}\b"),
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
  text
    .lines()
    .enumerate()
    .flat_map(|(number, line)| {
      SHAPES
        .iter()
        .filter(|(_, shape)| shape.is_match(line))
        .map(move |(word, _)| format!("{path}:{}: {word}", number + 1))
    })
    .collect()
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
at 10.0.0.12
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
      "sample:3: an IP address",
      "sample:4: a web address",
      "sample:5: a host name",
      "sample:6: a home path",
      "sample:7: an account handle",
    ]
  );
  assert!(hits("sample", "read `docs/lore/index.md`, id {{owner.id}}, a 12345 count").is_empty());
}
