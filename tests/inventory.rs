//! The content tree is the inventory: every file the inventory maps stands in the tree once, and the tree
//! holds no file the inventory does not map.

mod support;

use std::collections::BTreeSet;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use support::{content_files, read, shown};

/// The cells of every table in `markdown`, row by row, each cell as the text of its inline code and words.
/// A table's first row is its header.
fn tables(markdown: &str) -> Vec<Vec<Vec<String>>> {
  let mut tables = Vec::new();
  let mut rows: Vec<Vec<String>> = Vec::new();
  let mut cells: Vec<String> = Vec::new();
  let mut cell: Option<String> = None;
  for event in Parser::new_ext(markdown, Options::ENABLE_TABLES) {
    match event {
      Event::Start(Tag::Table(_)) => rows.clear(),
      Event::Start(Tag::TableHead | Tag::TableRow) => cells.clear(),
      Event::Start(Tag::TableCell) => cell = Some(String::new()),
      Event::Text(text) | Event::Code(text) => {
        if let Some(cell) = cell.as_mut() {
          cell.push_str(&text);
        }
      }
      Event::End(TagEnd::TableCell) => cells.extend(cell.take()),
      Event::End(TagEnd::TableHead | TagEnd::TableRow) => rows.push(std::mem::take(&mut cells)),
      Event::End(TagEnd::Table) => tables.push(std::mem::take(&mut rows)),
      _ => {}
    }
  }
  tables
}

/// The new paths the inventory maps: the second column of every table headed `Old path`, `New path`.
fn mapped(markdown: &str) -> Vec<String> {
  tables(markdown)
    .into_iter()
    .filter(|table| {
      table
        .first()
        .is_some_and(|head| head.len() >= 2 && head[0] == "Old path" && head[1] == "New path")
    })
    .flat_map(|table| table.into_iter().skip(1).map(|row| row[1].clone()))
    .collect()
}

#[test]
fn the_tree_is_what_the_inventory_maps() {
  let listed = mapped(&read("docs/inventory.md"));
  let unique: BTreeSet<&String> = listed.iter().collect();
  assert_eq!(unique.len(), listed.len(), "the inventory maps a path twice: {listed:?}");

  let tree: BTreeSet<String> = content_files().iter().map(|path| shown(path)).collect();
  let listed: BTreeSet<String> = listed.into_iter().collect();
  let missing: Vec<&String> = listed.difference(&tree).collect();
  let extra: Vec<&String> = tree.difference(&listed).collect();
  assert!(missing.is_empty(), "the inventory maps files the tree lacks: {missing:?}");
  assert!(extra.is_empty(), "the tree holds files the inventory does not map: {extra:?}");
  assert_eq!(tree.len(), 86, "the content is 86 files");
}

#[test]
fn only_old_path_tables_count() {
  let sample = "\
| Old path | New path |
|---|---|
| `a/one.md` | `x/one.md` |
| `a/two.md` | `x/two.md` |

| Key | Stands for |
|---|---|
| `owner.name` | a name |
";
  assert_eq!(mapped(sample), ["x/one.md", "x/two.md"]);
}
