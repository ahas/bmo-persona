# Working in bmo-persona

These rules hold every change to this repository.

## The documents come first

- `docs/` is the canonical statement of what the repository holds and how it is read: `docs/format.md`
  for the shape of the layers, the lore index and the placeholders, `docs/boundary.md` for who receives
  the content and how, `docs/inventory.md` for every file, where it comes from and what replaced its
  personal values, and `docs/PERSONAL-DATA.md` for the values that never enter the repository.
- A change that would make a document wrong or incomplete repairs the document first, stating what is
  true now, and the content or the checks are changed to match. A file added, removed or renamed is
  added to, removed from or renamed in the inventory in the same commit.
- A document states what is true now: no change history, no rationale told through an earlier design,
  no pasted source, signature or config fragment, and no file-and-line pins.

## Boundaries

- **The repository is content, not a service.** It runs no server, exposes no API and calls no unit. No
  unit's code reads it by its path: a deployment copies the content into the tree of the unit that reads
  it (`docs/boundary.md`).
- **Dependencies:** registry crates for the checks, and nothing from another repository, the base
  libraries included. The dependency check in `tests/` holds this.
- **No hand-written parser for a general format.** Markdown and TOML are read by a maintained library;
  the index lines, tone lines and placeholders are this repository's own syntax and are read by pattern.

## Language

- **The content keeps its own language.** The layers and the lore are written as their readers read
  them, Korean included, and are never translated here.
- **Everything else is English:** the checks, their messages, the documents, `README.md`, this file,
  commit messages and branch names. No Korean in code or tests; the Hangul check in `tests/` and the
  testbed's `korean` check hold this.
- **No personal data anywhere.** A personal value in the content is a placeholder whose value the
  deployment keeps (`docs/format.md`, 「Placeholders」); the personal-value check in `tests/` holds the
  shapes such a value takes, and `docs/PERSONAL-DATA.md` states the rule. Outside the content no agent
  is named, and no user, home directory or machine either.

## Layout

- One Cargo package at the root holding only the checks, no other build tool's manifest, and no
  directory named after its parent; the layout check in `tests/` holds these.
- The content lives in `layers/` and `lore/`; the checks live in `tests/`.
- **`.graph/` is the code dependency graph**, written only by `bmo graph` in this checkout. A commit that
  adds, moves or removes a node's files brings its node along in the same commit.

## The stages of a change

1. **Writing:** for the checks, the only check is `cargo check --all-targets`; for the content, none.
2. **Review:** read the whole diff against the document governing it, then `cargo fmt --check` and
   `cargo clippy --all-targets -- -D warnings`.
3. **Verification:** the test targets touched, then `cargo test` once, output written to a file and the
   file read.

A change is not done before stage 3 has passed.
