//! What the tests share: the root, the walk over the repository's files, and the content's own syntax as
//! `docs/format.md` states it.

// Each test file is its own crate and uses only part of this module.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// The two content folders.
pub const CONTENT: [&str; 2] = ["layers", "lore"];

/// Directories no check reads: build output and git's own.
const SKIPPED: [&str; 2] = ["target", ".git"];

pub fn is_skipped(name: &str) -> bool {
  SKIPPED.contains(&name)
}

/// Every file under `dir`, at any depth, skipping build output and git's directory. A directory that
/// does not exist holds no files.
pub fn files_under(dir: &Path) -> Vec<PathBuf> {
  let mut files = Vec::new();
  if dir.is_dir() {
    collect(dir, &mut files);
  }
  files.sort();
  files
}

fn collect(dir: &Path, files: &mut Vec<PathBuf>) {
  for entry in fs::read_dir(dir).expect("a directory is readable") {
    let entry = entry.expect("an entry is readable");
    let kind = entry.file_type().expect("an entry's type is readable");
    if kind.is_dir() {
      if !is_skipped(&entry.file_name().to_string_lossy()) {
        collect(&entry.path(), files);
      }
    } else if kind.is_file() {
      files.push(entry.path());
    }
  }
}

/// A path as it reads relative to the root, with forward slashes, for comparing and for a failure's
/// message.
pub fn shown(path: &Path) -> String {
  let relative = path.strip_prefix(ROOT).unwrap_or(path);
  relative
    .components()
    .map(|part| part.as_os_str().to_string_lossy())
    .collect::<Vec<_>>()
    .join("/")
}

/// Every file of the content folders.
pub fn content_files() -> Vec<PathBuf> {
  CONTENT.iter().flat_map(|dir| files_under(&Path::new(ROOT).join(dir))).collect()
}

/// A file of the repository, read as text.
pub fn read(relative: &str) -> String {
  let path = Path::new(ROOT).join(relative);
  fs::read_to_string(&path).unwrap_or_else(|error| panic!("{relative} reads: {error}"))
}

/// One lore index entry.
#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
  pub slug: String,
  pub scope: Option<String>,
  pub title: String,
  pub position: usize,
}

/// An index line: a dash and a space, the slug, an optional bracketed scope tag, then an em dash between
/// white space and the title to the end of the line.
static ENTRY: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^- ([a-z0-9-]+)(?:\s+(\[[^\]]*\]))?\s+\u{2014}\s+(.+)$").expect("an index line")
});

/// Every line of `index` that is an entry, in order, slug and all, duplicates included.
pub fn entry_lines(index: &str) -> Vec<String> {
  index
    .lines()
    .filter_map(|line| ENTRY.captures(line).map(|found| found[1].to_owned()))
    .collect()
}

/// The entries of `index` as a reader takes them: the first line with a slug wins, positions count the
/// kept entries from zero, and a line of any other shape is skipped.
pub fn entries(index: &str) -> Vec<Entry> {
  let mut kept: Vec<Entry> = Vec::new();
  for line in index.lines() {
    let Some(found) = ENTRY.captures(line) else { continue };
    if kept.iter().any(|entry| entry.slug == found[1]) {
      continue;
    }
    kept.push(Entry {
      slug: found[1].to_owned(),
      scope: found.get(2).map(|tag| tag.as_str().to_owned()),
      title: found[3].to_owned(),
      position: kept.len(),
    });
  }
  kept
}

/// A slug: one or more lowercase ASCII letters, digits and hyphens.
pub fn is_slug(text: &str) -> bool {
  !text.is_empty() && text.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// A room layer's opening tone line: `tone:`, spaces or tabs, one name with no white space, spaces or tabs,
/// then the end of the line.
static TONE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^tone:[ \t]*(\S+)[ \t]*(?:\n|$)").expect("a tone line"));

/// The tone a room layer's text opens with, read from the text trimmed as its reader trims it; `None`
/// where it has no tone line.
pub fn tone(room: &str) -> Option<String> {
  TONE.captures(room.trim()).map(|found| found[1].to_owned())
}

/// A placeholder: two opening braces, a key of dotted lowercase words, two closing braces.
pub static PLACEHOLDER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\{\{([a-z0-9-]+(?:\.[a-z0-9-]+)*)\}\}").expect("a placeholder"));
