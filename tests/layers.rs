//! The persona layers: the base layer is there, and every room layer's tone line names a tone layer that
//! exists, or none.

mod support;

use std::path::Path;

use support::{ROOT, files_under, read, shown, tone};

/// The tone layers: every layer file beside the base layer, named by its tone.
fn tone_layers() -> Vec<String> {
  files_under(&Path::new(ROOT).join("layers"))
    .iter()
    .filter(|path| path.parent().is_some_and(|dir| dir == Path::new(ROOT).join("layers")))
    .filter_map(|path| path.file_stem().map(|stem| stem.to_string_lossy().into_owned()))
    .filter(|name| name != "base")
    .collect()
}

#[test]
fn the_base_layer_holds_a_persona() {
  assert!(!read("layers/base.md").trim().is_empty(), "the base layer is empty");
}

#[test]
fn there_are_tone_layers() {
  assert!(!tone_layers().is_empty(), "no tone layer stands beside the base layer");
}

#[test]
fn every_tone_line_names_a_tone_layer_or_none() {
  let tones = tone_layers();
  let rooms = files_under(&Path::new(ROOT).join("layers/rooms"));
  assert!(!rooms.is_empty(), "the room layers are there");
  let broken: Vec<String> = rooms
    .iter()
    .filter_map(|path| {
      let named = tone(&read(&shown(path)))?;
      (named != "none" && !tones.contains(&named)).then(|| format!("{}: {named}", shown(path)))
    })
    .collect();
  assert!(broken.is_empty(), "tone lines naming no tone layer: {broken:?}");
}

#[test]
fn the_tone_line_reads_as_the_format_says() {
  assert_eq!(tone("tone: warm\n\n# A room").as_deref(), Some("warm"));
  assert_eq!(tone("\n  tone:\tnone \n").as_deref(), Some("none"));
  assert_eq!(tone("tone:plain").as_deref(), Some("plain"));
  assert_eq!(tone("# A room\ntone: warm"), None);
  assert_eq!(tone("tone: two words"), None);
  assert_eq!(tone("Tone: warm"), None);
}
