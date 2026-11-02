//! Dependencies: every one a registry crate. None comes by path or from a git source, and none is another
//! unit's crate; the repository depends on no other unit, not even the base libraries.

use crate::support;

use support::read;
use toml::{Table, Value};

/// The dependency tables a manifest or a target section may carry.
const DEPENDENCY_TABLES: [&str; 5] = [
  "dependencies",
  "dev-dependencies",
  "build-dependencies",
  "dev_dependencies",
  "build_dependencies",
];

/// The dependency tables standing directly in one section of a manifest.
fn own(section: &Table) -> impl Iterator<Item = &Table> {
  DEPENDENCY_TABLES.iter().filter_map(|name| section.get(*name).and_then(Value::as_table))
}

/// Every dependency table in a manifest: the package's own, each target's, a workspace's shared one, and
/// each patch source's.
fn dependency_tables(manifest: &Table) -> Vec<&Table> {
  let mut tables: Vec<&Table> = own(manifest).collect();
  if let Some(targets) = manifest.get("target").and_then(Value::as_table) {
    for target in targets.values().filter_map(Value::as_table) {
      tables.extend(own(target));
    }
  }
  if let Some(shared) = manifest
    .get("workspace")
    .and_then(|workspace| workspace.get("dependencies"))
    .and_then(Value::as_table)
  {
    tables.push(shared);
  }
  if let Some(patches) = manifest.get("patch").and_then(Value::as_table) {
    tables.extend(patches.values().filter_map(Value::as_table));
  }
  tables
}

/// Each dependency of `manifest` that is not a registry crate of no other unit.
fn violations(manifest: &Table) -> Vec<String> {
  let mut found = Vec::new();
  for table in dependency_tables(manifest) {
    for (key, spec) in table {
      let detail = spec.as_table();
      let name = detail.and_then(|d| d.get("package")).and_then(Value::as_str).unwrap_or(key);
      if detail.is_some_and(|d| d.contains_key("path")) {
        found.push(format!("{key} comes by path"));
      } else if detail.is_some_and(|d| d.contains_key("git")) {
        found.push(format!("{key} comes from a git source"));
      } else if name.starts_with("bmo-") {
        found.push(format!("{key} is another unit's crate ({name})"));
      }
    }
  }
  found
}

#[test]
fn every_dependency_is_a_registry_crate() {
  let manifest: Table = read("Cargo.toml").parse().expect("the manifest reads");
  let found = violations(&manifest);
  assert!(found.is_empty(), "these dependencies are not registry crates: {found:?}");
}

#[test]
fn a_manifest_breaking_the_rule_is_named() {
  let manifest: Table = r#"
    [dependencies]
    serde = "1"
    bmo-chat = { version = "0.1" }
    renamed = { package = "bmo-receiver", version = "0.1" }
    inside = { path = "crates/inside" }
    remote = { git = "https://example.invalid/remote" }

    [target.'cfg(unix)'.dev-dependencies]
    bmo-ui = "0.1"

    [workspace.dependencies]
    regex = "1"
  "#
  .parse()
  .expect("the sample reads");
  let mut found = violations(&manifest);
  found.sort();
  assert_eq!(
    found,
    [
      "bmo-chat is another unit's crate (bmo-chat)",
      "bmo-ui is another unit's crate (bmo-ui)",
      "inside comes by path",
      "remote comes from a git source",
      "renamed is another unit's crate (bmo-receiver)",
    ]
  );
}
