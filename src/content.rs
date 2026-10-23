//! The content as `docs/format.md` states it: the persona layers with their tone lines, and the lore index
//! with its entries, its sections and its chunks.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use regex::Regex;

use crate::Error;

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

/// The lines of `text`, each with the byte offset it starts at, ended as `str::lines` ends them.
fn lines_at(text: &str) -> impl Iterator<Item = (usize, &str)> {
  text.split_inclusive('\n').scan(0, |offset, line| {
    let at = *offset;
    *offset += line.len();
    let line = line.strip_suffix('\n').unwrap_or(line);
    Some((at, line.strip_suffix('\r').unwrap_or(line)))
  })
}

/// The entries of `index` as a reader takes them, each with the byte offset of its line.
fn located(index: &str) -> Vec<(usize, Entry)> {
  let mut kept: Vec<(usize, Entry)> = Vec::new();
  for (at, line) in lines_at(index) {
    let Some(found) = ENTRY.captures(line) else { continue };
    if kept.iter().any(|(_, entry)| entry.slug == found[1]) {
      continue;
    }
    let entry = Entry {
      slug: found[1].to_owned(),
      scope: found.get(2).map(|tag| tag.as_str().to_owned()),
      title: found[3].to_owned(),
      position: kept.len(),
    };
    kept.push((at, entry));
  }
  kept
}

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
  located(index).into_iter().map(|(_, entry)| entry).collect()
}

/// A section of the index: a level-two heading and the entries under it, or no heading for the entries
/// above every such heading.
#[derive(Debug, PartialEq, Eq)]
pub struct Section {
  pub heading: Option<String>,
  pub entries: Vec<Entry>,
}

/// The level-two headings of `markdown`, each with the byte offset it starts at and its text trimmed.
fn headings(markdown: &str) -> Vec<(usize, String)> {
  let mut found = Vec::new();
  let mut open: Option<(usize, String)> = None;
  for (event, range) in Parser::new(markdown).into_offset_iter() {
    match event {
      Event::Start(Tag::Heading {
        level: HeadingLevel::H2,
        ..
      }) => open = Some((range.start, String::new())),
      Event::Text(text) | Event::Code(text) => {
        if let Some((_, heading)) = open.as_mut() {
          heading.push_str(&text);
        }
      }
      Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
        found.extend(open.take().map(|(at, heading)| (at, heading.trim().to_owned())))
      }
      _ => {}
    }
  }
  found
}

/// The sections of `index` in the order their first entries appear: an entry belongs to the level-two
/// heading standing last above it, sections with the same heading are one, and a heading with no entry is
/// no section.
pub fn sections(index: &str) -> Vec<Section> {
  let headings = headings(index);
  let mut sections: Vec<Section> = Vec::new();
  for (at, entry) in located(index) {
    let heading = headings.iter().rev().find(|(start, _)| *start <= at).map(|(_, text)| text.clone());
    match sections.iter_mut().find(|section| section.heading == heading) {
      Some(section) => section.entries.push(entry),
      None => sections.push(Section {
        heading,
        entries: vec![entry],
      }),
    }
  }
  sections
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

/// A room layer as its reader takes it: the text trimmed, and after its tone line the rest, trimmed.
pub fn room_layer(room: &str) -> &str {
  let trimmed = room.trim();
  match TONE.find(trimmed) {
    Some(line) => trimmed[line.end()..].trim(),
    None => trimmed,
  }
}

/// What a persona layer is.
#[derive(Debug, PartialEq, Eq)]
pub enum LayerKind {
  Base,
  Tone(String),
  Room { slug: String, tone: Option<String> },
}

/// A persona layer as its reader takes it, with the path it is read from.
#[derive(Debug)]
pub struct PersonaLayer {
  pub kind: LayerKind,
  pub source: String,
  pub text: String,
}

/// A lore piece: an index entry with its chunk, and the path the chunk is read from.
#[derive(Debug)]
pub struct LorePiece {
  pub entry: Entry,
  pub source: String,
  pub text: String,
}

/// A section of the index with its pieces.
#[derive(Debug)]
pub struct LoreSection {
  pub heading: Option<String>,
  pub pieces: Vec<LorePiece>,
}

/// The content tree: the persona layers, the base layer first, then the tone layers by name, then the room
/// layers by slug; and the lore, section by section.
#[derive(Debug)]
pub struct Content {
  pub persona: Vec<PersonaLayer>,
  pub lore: Vec<LoreSection>,
}

/// A file of the tree, read as text.
fn text(root: &Path, relative: &str) -> Result<String, Error> {
  fs::read_to_string(root.join(relative)).map_err(|why| Error::at(relative, why))
}

/// A layer file, read as text; a missing file reads as empty.
fn layer_text(root: &Path, relative: &str) -> Result<String, Error> {
  match fs::read_to_string(root.join(relative)) {
    Err(why) if why.kind() == io::ErrorKind::NotFound => Ok(String::new()),
    read => read.map_err(|why| Error::at(relative, why)),
  }
}

/// The names, without `.md`, of the markdown files directly in `dir`, sorted; a missing directory holds
/// none.
fn markdown_names(root: &Path, dir: &str) -> Result<Vec<String>, Error> {
  let path: PathBuf = root.join(dir);
  if !path.is_dir() {
    return Ok(Vec::new());
  }
  let mut names = Vec::new();
  for entry in fs::read_dir(&path).map_err(|why| Error::at(dir, why))? {
    let entry = entry.map_err(|why| Error::at(dir, why))?;
    let name = entry.file_name().to_string_lossy().into_owned();
    if entry.path().is_file()
      && let Some(stem) = name.strip_suffix(".md")
    {
      names.push(stem.to_owned());
    }
  }
  names.sort();
  Ok(names)
}

impl Content {
  /// The content tree standing at `root`.
  pub fn read(root: &Path) -> Result<Self, Error> {
    let mut persona = vec![PersonaLayer {
      kind: LayerKind::Base,
      source: "layers/base.md".to_owned(),
      text: layer_text(root, "layers/base.md")?.trim().to_owned(),
    }];
    for name in markdown_names(root, "layers")?.into_iter().filter(|name| name != "base") {
      let source = format!("layers/{name}.md");
      let text = layer_text(root, &source)?.trim().to_owned();
      persona.push(PersonaLayer {
        kind: LayerKind::Tone(name),
        source,
        text,
      });
    }
    for slug in markdown_names(root, "layers/rooms")? {
      let source = format!("layers/rooms/{slug}.md");
      let room = layer_text(root, &source)?;
      let kind = LayerKind::Room {
        slug,
        tone: tone(&room),
      };
      persona.push(PersonaLayer {
        kind,
        source,
        text: room_layer(&room).to_owned(),
      });
    }

    let mut lore = Vec::new();
    for section in sections(&text(root, "lore/index.md")?) {
      let mut pieces = Vec::new();
      for entry in section.entries {
        let source = format!("lore/chunks/{}.md", entry.slug);
        let text = text(root, &source)?.trim().to_owned();
        pieces.push(LorePiece {
          entry,
          source,
          text,
        });
      }
      lore.push(LoreSection {
        heading: section.heading,
        pieces,
      });
    }
    Ok(Self { persona, lore })
  }
}
