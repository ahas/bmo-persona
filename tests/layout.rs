//! The repository's layout: one Cargo manifest at the root and no other build tool's, and no directory
//! named after its parent.

mod support;

use std::fs;
use std::path::Path;

use support::{ROOT, is_skipped, shown};

/// File names that declare a package or a project to some other build tool.
const OTHER_MANIFESTS: [&str; 12] = [
  "package.json",
  "bun.lock",
  "bun.lockb",
  "deno.json",
  "pnpm-workspace.yaml",
  "pyproject.toml",
  "setup.py",
  "go.mod",
  "Gemfile",
  "pom.xml",
  "build.gradle",
  "Makefile",
];

fn manifests_and_nesting(dir: &Path, found: &mut Vec<String>) {
  for entry in fs::read_dir(dir).expect("a directory is readable") {
    let entry = entry.expect("an entry is readable");
    let name = entry.file_name().to_string_lossy().into_owned();
    let path = entry.path();
    if path.is_dir() {
      if is_skipped(&name) {
        continue;
      }
      if dir.file_name().is_some_and(|parent| parent.to_string_lossy() == name) {
        found.push(format!("{} is named after its parent", shown(&path)));
      }
      manifests_and_nesting(&path, found);
    } else if OTHER_MANIFESTS.contains(&name.as_str()) || (name == "Cargo.toml" && dir != Path::new(ROOT)) {
      found.push(format!("{} is a manifest besides the root's", shown(&path)));
    }
  }
}

#[test]
fn one_manifest_and_no_nesting() {
  let mut found = Vec::new();
  manifests_and_nesting(Path::new(ROOT), &mut found);
  assert!(found.is_empty(), "the layout breaks its rules: {found:?}");
  assert!(Path::new(ROOT).join("Cargo.toml").is_file(), "the root holds the manifest");
}
