//! The installer: the brain file it writes from the repository's own content holds a piece per layer file
//! and per index entry, its layers in their order, and the same keys every time it runs
//! (`docs/boundary.md`, 「The brain file」).

use crate::support;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use bmo_persona::brain_file::{LORE_IMPORTANCE, PERSONA_IMPORTANCE, SHAPE};
use bmo_persona::content::{Entry, Section, entries, room_layer, sections, tone};
use rusqlite::{Connection, OpenFlags};
use support::{ROOT, files_under, read};

/// A directory of this test run's own under the temporary directory, empty.
fn scratch(what: &str) -> PathBuf {
  let dir = std::env::temp_dir().join(format!("bmo-persona-install-{what}-{}", std::process::id()));
  let _ = fs::remove_dir_all(&dir);
  fs::create_dir_all(&dir).expect("the scratch directory is made");
  dir
}

/// The installer run over the repository's content, writing `out`.
fn install(out: &Path, name: Option<&str>) {
  let mut command = Command::new(env!("CARGO_BIN_EXE_bmo-persona-install"));
  command.arg("--root").arg(ROOT).arg("--out").arg(out);
  if let Some(name) = name {
    command.arg("--name").arg(name);
  }
  let run = command.output().expect("the installer runs");
  assert!(run.status.success(), "the installer fails: {}", String::from_utf8_lossy(&run.stderr));
}

fn open(path: &Path) -> Connection {
  Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY).expect("the brain file opens")
}

fn count(file: &Connection, sql: &str) -> usize {
  file.query_row(sql, [], |row| row.get::<_, i64>(0)).expect("a count reads") as usize
}

/// A memory as the file holds it, with its layer's name.
#[derive(Debug, PartialEq)]
struct Row {
  key: String,
  kind: String,
  text: String,
  layer: String,
  position: i64,
  status: String,
  importance: i64,
  hits: i64,
  strength: f64,
  own_lineage: bool,
  live: bool,
  labels: BTreeMap<String, String>,
}

/// Every memory of the file, in id order.
fn rows(file: &Connection) -> Vec<Row> {
  let mut query = file
    .prepare(
      "SELECT m.key, m.type, m.text, l.name, m.position, m.status, m.importance, m.hits, m.strength, \
       m.lineage_id = m.id, m.superseded_by IS NULL AND m.valid_to IS NULL, m.labels \
       FROM memories m JOIN layers l ON l.id = m.layer_id ORDER BY m.id",
    )
    .expect("the memories query");
  query
    .query_map([], |row| {
      let labels: String = row.get(11)?;
      Ok(Row {
        key: row.get(0)?,
        kind: row.get(1)?,
        text: row.get(2)?,
        layer: row.get(3)?,
        position: row.get(4)?,
        status: row.get(5)?,
        importance: row.get(6)?,
        hits: row.get(7)?,
        strength: row.get(8)?,
        own_lineage: row.get(9)?,
        live: row.get(10)?,
        labels: serde_json::from_str(&labels).expect("the labels are a map of strings"),
      })
    })
    .expect("the memories read")
    .collect::<Result<_, _>>()
    .expect("every memory reads")
}

/// The file's layers by position, each name with its position.
fn layers(file: &Connection) -> Vec<(String, i64)> {
  let mut query = file
    .prepare("SELECT name, position FROM layers ORDER BY position, id")
    .expect("a query");
  query
    .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
    .expect("the layers read")
    .collect::<Result<_, _>>()
    .expect("every layer reads")
}

/// The stems of the markdown files directly in `dir`, sorted.
fn stems(dir: &str) -> Vec<String> {
  let dir = Path::new(ROOT).join(dir);
  files_under(&dir)
    .iter()
    .filter(|path| path.parent() == Some(dir.as_path()))
    .filter_map(|path| path.file_name()?.to_str()?.strip_suffix(".md").map(str::to_owned))
    .collect()
}

#[test]
fn the_brain_file_holds_the_content() {
  let dir = scratch("content");
  let out = dir.join("persona.sqlite");
  install(&out, Some("a persona"));
  let file = open(&out);

  let (shape, name): (i64, String) = file
    .query_row("SELECT shape, name FROM brain_file", [], |row| Ok((row.get(0)?, row.get(1)?)))
    .unwrap();
  assert_eq!((shape, name.as_str()), (SHAPE, "a persona"));
  assert_eq!(count(&file, "SELECT count(*) FROM brain_file"), 1);
  assert_eq!(count(&file, "SELECT count(*) FROM subjects"), 0, "no subject is written");
  assert_eq!(count(&file, "SELECT count(*) FROM memory_subjects"), 0, "no memory is linked");

  let tones: Vec<String> = stems("layers").into_iter().filter(|stem| stem != "base").collect();
  let rooms = stems("layers/rooms");
  let index = read("lore/index.md");
  let headings: Vec<String> =
    index.lines().filter_map(|line| line.strip_prefix("## ")).map(str::to_owned).collect();
  let mut expected = vec!["persona".to_owned()];
  expected.extend(tones.iter().map(|tone| format!("persona/tone/{tone}")));
  expected.extend(rooms.iter().map(|room| format!("persona/room/{room}")));
  expected.extend(headings.iter().map(|heading| format!("lore/{heading}")));
  let layers = layers(&file);
  let names: Vec<String> = layers.iter().map(|(name, _)| name.clone()).collect();
  assert_eq!(names, expected, "the layers stand in their order");
  let positions: Vec<i64> = layers.iter().map(|(_, position)| *position).collect();
  assert_eq!(
    positions,
    (0..layers.len() as i64).collect::<Vec<_>>(),
    "the layers are positioned from zero"
  );

  let rows = rows(&file);
  let persona: Vec<&Row> = rows.iter().filter(|row| row.kind == "persona").collect();
  let lore: Vec<&Row> = rows.iter().filter(|row| row.kind == "lore").collect();
  let mut sources = vec!["layers/base.md".to_owned()];
  sources.extend(tones.iter().map(|tone| format!("layers/{tone}.md")));
  let plain = sources.iter().filter(|source| !read(source).trim().is_empty()).count();
  let rooms_with_text = rooms
    .iter()
    .filter(|room| !room_layer(&read(&format!("layers/rooms/{room}.md"))).is_empty())
    .count();
  assert_eq!(persona.len(), plain + rooms_with_text, "a persona piece per layer file with text");
  assert_eq!(lore.len(), stems("lore/chunks").len(), "a lore piece per chunk");
  assert_eq!(lore.len(), entries(&index).len(), "a lore piece per index entry");
  assert_eq!(rows.len(), persona.len() + lore.len(), "no row of another type");

  for row in &rows {
    assert_eq!(row.status, "active", "{} is active", row.key);
    assert!(row.own_lineage && row.live, "{} is a first, live version", row.key);
    assert_eq!((row.hits, row.strength), (0, 1.0), "{} is unread", row.key);
  }

  for row in &persona {
    assert_eq!(row.key, row.layer, "a persona piece is keyed by its layer");
    assert_eq!((row.position, row.importance), (0, PERSONA_IMPORTANCE));
    let source = &row.labels["source"];
    let file_text = read(source);
    if row.key.starts_with("persona/room/") {
      assert_eq!(row.labels.get("tone"), tone(&file_text).as_ref(), "{source} carries its tone");
      assert!(!row.text.starts_with("tone:"), "{source} leaves its tone line out");
      assert!(file_text.trim().ends_with(&row.text), "{source} is taken whole after its tone line");
    } else {
      assert_eq!(row.text, file_text.trim(), "{source} is taken trimmed");
    }
  }
  assert_eq!(persona[0].labels["source"], "layers/base.md", "the base layer comes first");

  let by_key: BTreeMap<&str, &Row> = lore.iter().map(|row| (row.key.as_str(), *row)).collect();
  for Entry {
    slug,
    scope,
    title,
    position,
  } in entries(&index)
  {
    let row = by_key
      .get(format!("lore/{slug}").as_str())
      .unwrap_or_else(|| panic!("{slug} has a piece"));
    let source = format!("lore/chunks/{slug}.md");
    assert_eq!(row.labels["source"], source);
    assert_eq!(row.labels["title"], title);
    assert_eq!(row.labels.get("scope"), scope.as_ref());
    assert_eq!(row.text, read(&source).trim(), "{source} is taken trimmed");
    assert_eq!((row.position, row.importance), (position as i64, LORE_IMPORTANCE));
    assert!(row.layer.starts_with("lore/"), "{slug} stands in a section's layer");
  }

  let _ = fs::remove_dir_all(&dir);
}

#[test]
fn two_runs_give_the_same_keys() {
  let dir = scratch("twice");
  let first = dir.join("first.sqlite");
  let second = dir.join("second.sqlite");
  install(&first, None);
  install(&second, None);
  install(&second, None);
  let (first, second) = (open(&first), open(&second));
  let name: String = second.query_row("SELECT name FROM brain_file", [], |row| row.get(0)).unwrap();
  assert_eq!(name, "bmo-persona", "the file is named after the package where no name is given");
  assert_eq!(layers(&first), layers(&second), "the layers are the same");
  let (first, second) = (rows(&first), rows(&second));
  assert_eq!(first, second, "the memories are the same, key for key");
  let mut keys: Vec<&String> = first.iter().map(|row| &row.key).collect();
  keys.sort();
  keys.dedup();
  assert_eq!(keys.len(), first.len(), "no key is written twice");
  let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_index_sections_read_as_the_boundary_says() {
  let index = "\
# Lore

- above \u{2014} Above every section

## The `first` part

- one \u{2014} One
- two \u{2014} Two

## Empty part

Prose only.

## The `first` part

- three \u{2014} Three
- one \u{2014} A later line with the same slug
";
  let entry = |slug: &str, position| Entry {
    slug: slug.to_owned(),
    scope: None,
    title: match slug {
      "above" => "Above every section".to_owned(),
      other => format!("{}{}", other[..1].to_uppercase(), &other[1..]),
    },
    position,
  };
  assert_eq!(
    sections(index),
    [
      Section {
        heading: None,
        entries: vec![entry("above", 0)]
      },
      Section {
        heading: Some("The first part".to_owned()),
        entries: vec![entry("one", 1), entry("two", 2), entry("three", 3)],
      },
    ]
  );
}
