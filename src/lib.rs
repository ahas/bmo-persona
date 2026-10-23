//! The content read as `docs/format.md` states it, and written as the brain file the installer hands the
//! operator (`docs/boundary.md`, 「The brain file」).

pub mod brain_file;
pub mod content;

use std::fmt;

/// What stopped a read or a write, with the path or the step it stopped at.
#[derive(Debug)]
pub struct Error(String);

impl Error {
  fn at(what: impl fmt::Display, why: impl fmt::Display) -> Self {
    Self(format!("{what}: {why}"))
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl std::error::Error for Error {}
