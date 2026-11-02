//! The repository's layout: one Cargo manifest at the root and no other build tool's, no directory named
//! after its parent, and one version.

use crate::support;

use std::fs;
use std::path::Path;

use support::{ROOT, is_skipped, read, shown};

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

/// `MAJOR.MINOR.PATCH`, each a number with no leading zero.
fn is_semver(text: &str) -> bool {
  let parts: Vec<&str> = text.split('.').collect();
  parts.len() == 3
    && parts.iter().all(|part| {
      !part.is_empty()
        && part.bytes().all(|b| b.is_ascii_digit())
        && (part.len() == 1 || !part.starts_with('0'))
    })
}

#[test]
fn one_version_in_semver() {
  let version = read("VERSION");
  let version = version.trim_end();
  assert!(is_semver(version), "VERSION is no semantic version: {version:?}");
  let manifest: toml::Table = read("Cargo.toml").parse().expect("the manifest reads");
  let package = manifest
    .get("package")
    .and_then(|package| package.get("version"))
    .and_then(|v| v.as_str());
  assert_eq!(package, Some(version), "the package's version is VERSION's");
}

#[test]
fn the_semver_shape() {
  assert!(is_semver("0.1.0") && is_semver("10.0.12"));
  assert!(!is_semver("0.1") && !is_semver("01.0.0") && !is_semver("1.0.0-rc1") && !is_semver("1..0"));
}

#[test]
fn one_manifest_and_no_nesting() {
  let mut found = Vec::new();
  manifests_and_nesting(Path::new(ROOT), &mut found);
  assert!(found.is_empty(), "the layout breaks its rules: {found:?}");
  assert!(Path::new(ROOT).join("Cargo.toml").is_file(), "the root holds the manifest");
}
