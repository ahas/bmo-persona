//! The brain file (`docs/boundary.md`, 「The brain file」): the content as layers and memories, written as a
//! SQLite database in the shape the brain's boundary states for its brain files.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, params};

use crate::Error;
use crate::content::{Content, LayerKind};

/// The version of the file's shape the brain reads.
pub const SHAPE: i64 = 1;

/// Who the file's memories are written by.
pub const ACTOR: &str = "bmo-persona-install";

/// The weight of a persona piece: high, for the agent's own persona.
pub const PERSONA_IMPORTANCE: i64 = 8;

/// The weight of a lore piece: middling, for reference read when a turn needs it.
pub const LORE_IMPORTANCE: i64 = 5;

/// The file's tables, in the brain file's shape.
const TABLES: &str = "
CREATE TABLE brain_file (shape INTEGER NOT NULL, exported_at INTEGER NOT NULL, name TEXT NOT NULL);
CREATE TABLE subjects (id INTEGER PRIMARY KEY, type TEXT NOT NULL, key TEXT NOT NULL, description TEXT NOT NULL);
CREATE TABLE layers (id INTEGER PRIMARY KEY, name TEXT NOT NULL, description TEXT NOT NULL, \
  position INTEGER NOT NULL);
CREATE TABLE memories (id INTEGER PRIMARY KEY, key TEXT, type TEXT NOT NULL, text TEXT NOT NULL, \
  layer_id INTEGER, position INTEGER NOT NULL, status TEXT NOT NULL, lineage_id INTEGER NOT NULL, \
  superseded_by INTEGER, valid_from INTEGER NOT NULL, valid_to INTEGER, importance INTEGER NOT NULL, \
  hits INTEGER NOT NULL, last_hit_at INTEGER, last_confirmed_at INTEGER, strength REAL NOT NULL, \
  actor_name TEXT NOT NULL, labels TEXT NOT NULL, created_at INTEGER NOT NULL);
CREATE TABLE memory_subjects (memory_id INTEGER NOT NULL, subject_id INTEGER NOT NULL, \
  PRIMARY KEY (memory_id, subject_id));
";

/// A layer of the file; its position is its place among the file's layers.
#[derive(Debug, PartialEq, Eq)]
pub struct Layer {
  pub name: String,
  pub description: String,
}

/// A memory of the file, in the layer at its place among the file's layers.
#[derive(Debug, PartialEq, Eq)]
pub struct Memory {
  pub key: String,
  pub kind: &'static str,
  pub text: String,
  pub layer: usize,
  pub position: i64,
  pub importance: i64,
  pub labels: BTreeMap<String, String>,
}

/// What the file holds: its name, its layers in order and its memories. It links no memory to a subject.
#[derive(Debug)]
pub struct BrainFile {
  pub name: String,
  pub layers: Vec<Layer>,
  pub memories: Vec<Memory>,
}

/// A persona layer's name and description.
fn persona_layer(kind: &LayerKind) -> Layer {
  let (name, description) = match kind {
    LayerKind::Base => ("persona".to_owned(), "The base persona.".to_owned()),
    LayerKind::Tone(tone) => (
      format!("persona/tone/{tone}"),
      format!("The {tone} tone layer, laid over the base persona."),
    ),
    LayerKind::Room { slug, tone } => {
      let laid = match tone.as_deref() {
        Some("none") => "taking no tone layer".to_owned(),
        Some(tone) => format!("laid over the {tone} tone layer"),
        None => "taking its reader's default tone".to_owned(),
      };
      (format!("persona/room/{slug}"), format!("The {slug} room layer, {laid}."))
    }
  };
  Layer { name, description }
}

/// A lore section's layer: `lore/<heading>`, or `lore` for the entries no section holds.
fn lore_layer(heading: Option<&str>) -> Layer {
  match heading {
    Some(heading) => Layer {
      name: format!("lore/{heading}"),
      description: heading.to_owned(),
    },
    None => Layer {
      name: "lore".to_owned(),
      description: "The lore library.".to_owned(),
    },
  }
}

impl BrainFile {
  /// The file the content gives, named `name`.
  pub fn of(content: &Content, name: &str) -> Self {
    let mut layers = Vec::new();
    let mut memories = Vec::new();
    for layer in &content.persona {
      let named = persona_layer(&layer.kind);
      if layer.text.is_empty() {
        layers.push(named);
        continue;
      }
      let mut labels = BTreeMap::from([("source".to_owned(), layer.source.clone())]);
      if let LayerKind::Room {
        tone: Some(tone), ..
      } = &layer.kind
      {
        labels.insert("tone".to_owned(), tone.clone());
      }
      memories.push(Memory {
        key: named.name.clone(),
        kind: "persona",
        text: layer.text.clone(),
        layer: layers.len(),
        position: 0,
        importance: PERSONA_IMPORTANCE,
        labels,
      });
      layers.push(named);
    }
    for section in &content.lore {
      let named = lore_layer(section.heading.as_deref());
      let at = match layers.iter().position(|layer| layer.name == named.name) {
        Some(at) => at,
        None => {
          layers.push(named);
          layers.len() - 1
        }
      };
      for piece in &section.pieces {
        let mut labels = BTreeMap::from([
          ("source".to_owned(), piece.source.clone()),
          ("title".to_owned(), piece.entry.title.clone()),
        ]);
        if let Some(scope) = &piece.entry.scope {
          labels.insert("scope".to_owned(), scope.clone());
        }
        memories.push(Memory {
          key: format!("lore/{}", piece.entry.slug),
          kind: "lore",
          text: piece.text.clone(),
          layer: at,
          position: piece.entry.position as i64,
          importance: LORE_IMPORTANCE,
          labels,
        });
      }
    }
    Self {
      name: name.to_owned(),
      layers,
      memories,
    }
  }

  /// The file written at `path`, every memory valid from `at`, in milliseconds since the epoch. It is
  /// written beside `path` first and then put in its place, replacing a file standing there.
  pub fn write(&self, path: &Path, at: i64) -> Result<(), Error> {
    let shown = path.display();
    let partial = beside(path);
    match fs::remove_file(&partial) {
      Err(why) if why.kind() != io::ErrorKind::NotFound => return Err(Error::at(partial.display(), why)),
      _ => {}
    }
    let mut file = Connection::open(&partial).map_err(|why| Error::at(&shown, why))?;
    self.fill(&mut file, at).map_err(|why| Error::at(&shown, why))?;
    file.close().map_err(|(_, why)| Error::at(&shown, why))?;
    fs::rename(&partial, path).map_err(|why| Error::at(&shown, why))
  }

  fn fill(&self, file: &mut Connection, at: i64) -> Result<(), Error> {
    file.execute_batch(TABLES).map_err(|why| Error::at("the tables", why))?;
    let tx = file.transaction().map_err(|why| Error::at("the transaction", why))?;
    tx.execute(
      "INSERT INTO brain_file (shape, exported_at, name) VALUES (?1, ?2, ?3)",
      params![SHAPE, at, self.name],
    )
    .map_err(|why| Error::at("brain_file", why))?;
    for (index, layer) in self.layers.iter().enumerate() {
      let id = index as i64 + 1;
      tx.execute(
        "INSERT INTO layers (id, name, description, position) VALUES (?1, ?2, ?3, ?4)",
        params![id, layer.name, layer.description, index as i64],
      )
      .map_err(|why| Error::at(&layer.name, why))?;
    }
    for (index, memory) in self.memories.iter().enumerate() {
      let id = index as i64 + 1;
      let labels = serde_json::to_string(&memory.labels).map_err(|why| Error::at(&memory.key, why))?;
      tx.execute(
        "INSERT INTO memories (id, key, type, text, layer_id, position, status, lineage_id, superseded_by, \
         valid_from, valid_to, importance, hits, last_hit_at, last_confirmed_at, strength, actor_name, labels, \
         created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'active', ?1, NULL, ?7, NULL, ?8, 0, NULL, NULL, 1.0, ?9, \
         ?10, ?7)",
        params![
          id,
          memory.key,
          memory.kind,
          memory.text,
          memory.layer as i64 + 1,
          memory.position,
          at,
          memory.importance,
          ACTOR,
          labels
        ],
      )
      .map_err(|why| Error::at(&memory.key, why))?;
    }
    tx.commit().map_err(|why| Error::at("the transaction", why))
  }
}

/// The path the file is written at before it is put in its place: `path` with `.partial` after its name.
fn beside(path: &Path) -> PathBuf {
  let mut name = OsString::from(path.as_os_str());
  name.push(".partial");
  PathBuf::from(name)
}
