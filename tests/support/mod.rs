//! What the tests share: the root, the walk over the repository's files, and the placeholder's syntax. The
//! rest of the content's own syntax is read by the library the installer reads it with.

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

/// A placeholder: two opening braces, a key of dotted lowercase words, two closing braces.
pub static PLACEHOLDER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\{\{([a-z0-9-]+(?:\.[a-z0-9-]+)*)\}\}").expect("a placeholder"));
