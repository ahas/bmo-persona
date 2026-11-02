//! Placeholders: only the documented keys appear, room keys only in room layers, none in the lore, and no
//! brace pair that is not a whole placeholder.

use crate::support;

use std::sync::LazyLock;

use regex::Regex;
use support::{PLACEHOLDER, content_files, read, shown};

/// The keys any layer may hold.
const EVERY_LAYER: [&str; 2] = ["owner.name", "owner.id"];

/// The keys a room layer may hold beside those: the room's own title, and each listed person's and bot's
/// fields, numbered from one.
static ROOM_KEY: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(
    r"^(?:room\.title|person-[1-9][0-9]*\.(?:name|id|called|age|relation)|bot-[1-9][0-9]*\.(?:name|id|handle))$",
  )
  .expect("a room key")
});

/// What is wrong with the placeholders of one file at `path`.
fn wrong(path: &str, text: &str) -> Vec<String> {
  let in_room = path.starts_with("layers/rooms/");
  let in_lore = path.starts_with("lore/");
  let mut found: Vec<String> = PLACEHOLDER
    .captures_iter(text)
    .map(|captured| captured[1].to_owned())
    .filter(|key| in_lore || !(EVERY_LAYER.contains(&key.as_str()) || (in_room && ROOM_KEY.is_match(key))))
    .map(|key| format!("{path}: {key}"))
    .collect();
  let left = PLACEHOLDER.replace_all(text, "");
  if left.contains("{{") || left.contains("}}") {
    found.push(format!("{path}: a brace pair that is no placeholder"));
  }
  found
}

#[test]
fn only_documented_placeholders_appear() {
  let found: Vec<String> = content_files()
    .iter()
    .flat_map(|path| {
      let path = shown(path);
      wrong(&path, &read(&path))
    })
    .collect();
  assert!(found.is_empty(), "placeholders out of place: {found:?}");
}

#[test]
fn the_rules_name_what_they_should() {
  assert!(wrong("layers/base.md", "Calls {{owner.name}}, id {{owner.id}}.").is_empty());
  assert!(
    wrong(
      "layers/rooms/room-a.md",
      "# {{room.title}}\n{{person-12.called}} {{bot-1.name}} {{bot-1.handle}}"
    )
    .is_empty()
  );
  assert_eq!(wrong("layers/base.md", "{{room.title}}"), ["layers/base.md: room.title"]);
  assert_eq!(
    wrong("layers/rooms/room-a.md", "{{person-0.name}}"),
    ["layers/rooms/room-a.md: person-0.name"]
  );
  assert_eq!(
    wrong("layers/rooms/room-a.md", "{{person-1.email}}"),
    ["layers/rooms/room-a.md: person-1.email"]
  );
  assert_eq!(
    wrong("layers/rooms/room-a.md", "{{bot-1.email}}"),
    ["layers/rooms/room-a.md: bot-1.email"]
  );
  assert_eq!(wrong("lore/index.md", "{{owner.name}}"), ["lore/index.md: owner.name"]);
  assert_eq!(
    wrong("layers/base.md", "{{Owner}}"),
    ["layers/base.md: a brace pair that is no placeholder"]
  );
}
