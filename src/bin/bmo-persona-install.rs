//! The installer: reads the content tree and writes it as a brain file, which the operator injects into a
//! brain with the brain's own route (`docs/boundary.md`, 「The brain file」).

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use bmo_persona::brain_file::BrainFile;
use bmo_persona::content::Content;

const USAGE: &str = "usage: bmo-persona-install --out <file> [--root <dir>] [--name <name>]";

/// What the command line asks for.
struct Options {
  out: PathBuf,
  root: PathBuf,
  name: String,
}

/// The options `args` give; `None` where they ask for the usage.
fn options(mut args: impl Iterator<Item = String>) -> Result<Option<Options>, String> {
  let mut out = None;
  let mut root = PathBuf::from(".");
  let mut name = env!("CARGO_PKG_NAME").to_owned();
  while let Some(flag) = args.next() {
    if flag == "--help" || flag == "-h" {
      return Ok(None);
    }
    let value = match flag.as_str() {
      "--out" | "--root" | "--name" => args.next().ok_or_else(|| format!("{flag} takes a value"))?,
      _ => return Err(format!("unknown argument {flag:?}")),
    };
    match flag.as_str() {
      "--out" => out = Some(PathBuf::from(value)),
      "--root" => root = PathBuf::from(value),
      _ => name = value,
    }
  }
  let out = out.ok_or("--out is required")?;
  Ok(Some(Options { out, root, name }))
}

/// Now, in milliseconds since the epoch.
fn now() -> i64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .map(|since| i64::try_from(since.as_millis()).unwrap_or(i64::MAX))
    .unwrap_or(0)
}

fn main() -> ExitCode {
  let options = match options(std::env::args().skip(1)) {
    Ok(Some(options)) => options,
    Ok(None) => {
      println!("{USAGE}");
      return ExitCode::SUCCESS;
    }
    Err(why) => {
      eprintln!("bmo-persona-install: {why}\n{USAGE}");
      return ExitCode::from(2);
    }
  };
  let written = Content::read(&options.root).and_then(|content| {
    let file = BrainFile::of(&content, &options.name);
    file.write(&options.out, now()).map(|()| file)
  });
  match written {
    Ok(file) => {
      let pieces = |kind| file.memories.iter().filter(|memory| memory.kind == kind).count();
      println!(
        "Wrote {}: {} layers, {} persona pieces, {} lore pieces, no subjects.",
        options.out.display(),
        file.layers.len(),
        pieces("persona"),
        pieces("lore")
      );
      ExitCode::SUCCESS
    }
    Err(why) => {
      eprintln!("bmo-persona-install: {why}");
      ExitCode::FAILURE
    }
  }
}
