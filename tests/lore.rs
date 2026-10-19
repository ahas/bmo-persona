//! The lore index and its chunks: every entry has its chunk, every chunk has its entry, and no slug is
//! listed twice or breaks the slug rule.

mod support;

use std::collections::BTreeSet;
use std::path::Path;

use support::{Entry, ROOT, entries, entry_lines, files_under, is_slug, read};

fn chunk_slugs() -> BTreeSet<String> {
  files_under(&Path::new(ROOT).join("lore/chunks"))
    .iter()
    .map(|path| {
      let name = path.file_name().expect("a chunk has a name").to_string_lossy().into_owned();
      name
        .strip_suffix(".md")
        .unwrap_or_else(|| panic!("a chunk is markdown: {name}"))
        .to_owned()
    })
    .collect()
}

#[test]
fn every_entry_has_its_chunk_and_every_chunk_its_entry() {
  let listed: BTreeSet<String> =
    entries(&read("lore/index.md")).into_iter().map(|entry| entry.slug).collect();
  let chunks = chunk_slugs();
  let without_chunk: Vec<&String> = listed.difference(&chunks).collect();
  let without_entry: Vec<&String> = chunks.difference(&listed).collect();
  assert!(without_chunk.is_empty(), "entries with no chunk: {without_chunk:?}");
  assert!(without_entry.is_empty(), "chunks with no entry: {without_entry:?}");
  assert!(!listed.is_empty(), "the index lists entries");
}

#[test]
fn no_slug_is_listed_twice() {
  let lines = entry_lines(&read("lore/index.md"));
  let mut seen = BTreeSet::new();
  let twice: Vec<&String> = lines.iter().filter(|slug| !seen.insert(*slug)).collect();
  assert!(twice.is_empty(), "slugs listed twice: {twice:?}");
}

#[test]
fn every_chunk_name_is_a_slug() {
  let broken: Vec<String> = chunk_slugs().into_iter().filter(|slug| !is_slug(slug)).collect();
  assert!(broken.is_empty(), "chunk names that are no slug: {broken:?}");
}

#[test]
fn the_index_grammar_reads_as_the_format_says() {
  let index = "\
# Lore

Prose before the entries.

- first-one \u{2014} The first title
- tagged [room] \u{2014} A scoped title
- Not-a-slug \u{2014} skipped, uppercase
- first-one \u{2014} A later line with the same slug
-missing-space \u{2014} skipped
- no-dash - skipped, a hyphen and not an em dash
- last \u{2014} The last title
";
  let entry = |slug: &str, scope: Option<&str>, title: &str, position| Entry {
    slug: slug.to_owned(),
    scope: scope.map(str::to_owned),
    title: title.to_owned(),
    position,
  };
  assert_eq!(
    entries(index),
    [
      entry("first-one", None, "The first title", 0),
      entry("tagged", Some("[room]"), "A scoped title", 1),
      entry("last", None, "The last title", 2),
    ]
  );
  assert_eq!(entry_lines(index), ["first-one", "tagged", "first-one", "last"]);
}

#[test]
fn the_slug_rule() {
  assert!(is_slug("ooo-geography") && is_slug("y5"));
  assert!(!is_slug("") && !is_slug("Upper") && !is_slug("under_score") && !is_slug("sp ace"));
}
