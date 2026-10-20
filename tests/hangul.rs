//! No Hangul in code or tests. The content keeps its own language and is not read here.

mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{ROOT, files_under, shown};

/// The Hangul syllables block's first and last code points, U+AC00 and U+D7A3, as numbers: an escape
/// would name a non-English letter, and the code holds none.
const FIRST_SYLLABLE: u32 = 0xAC00;
const LAST_SYLLABLE: u32 = 0xD7A3;

fn is_hangul(c: char) -> bool {
  (FIRST_SYLLABLE..=LAST_SYLLABLE).contains(&(c as u32))
}

/// Each line of `text` holding Hangul, as `path:line`.
fn hangul_lines(path: &str, text: &str) -> Vec<String> {
  text
    .lines()
    .enumerate()
    .filter(|(_, line)| line.chars().any(is_hangul))
    .map(|(number, _)| format!("{path}:{}", number + 1))
    .collect()
}

/// The code: the tests, and every manifest and declaration at the root.
fn code_files() -> Vec<PathBuf> {
  let mut files = files_under(&Path::new(ROOT).join("tests"));
  for entry in fs::read_dir(ROOT).expect("the repository root is readable") {
    let path = entry.expect("an entry is readable").path();
    if path.is_file() && path.extension().is_some_and(|ext| ext == "toml" || ext == "json") {
      files.push(path);
    }
  }
  files
}

#[test]
fn no_hangul_in_code_or_tests() {
  let found: Vec<String> = code_files()
    .iter()
    .flat_map(|path| {
      let bytes = fs::read(path).expect("a file is readable");
      hangul_lines(&shown(path), &String::from_utf8_lossy(&bytes))
    })
    .collect();
  assert!(found.is_empty(), "these lines hold Hangul: {found:?}");
}

#[test]
fn the_range_is_the_whole_syllables_block() {
  let first = char::from_u32(0xAC00).expect("a code point");
  let last = char::from_u32(0xD7A3).expect("a code point");
  let text = format!("ok\nsays {first}\nfine\nends {last}");
  assert_eq!(hangul_lines("sample", &text), ["sample:2", "sample:4"]);
  let outside = [char::from_u32(0xABFF), char::from_u32(0xD7A4)].map(|c| c.expect("a code point"));
  assert!(!outside.into_iter().any(is_hangul));
}
