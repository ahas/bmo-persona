**English** | [한국어](README.ko.md) | [日本語](README.ja.md) | [简体中文](README.zh-CN.md)

# bmo-persona

The persona layers and the lore library of a chat agent, kept as files. The base layer is the persona the
agent's system prompt carries; tone layers and room layers are laid over it room by room; the lore library
is an index of chunks the agent is offered as reference. Personal values in the layers are placeholders a
deployment fills in from its own configuration. The repository runs no service: its checks hold the content
to the format its readers rely on, a deployment places it where those readers look, and its installer writes
it as a brain file for the chat agent's brain.

## Layout

| Folder | Holds |
|---|---|
| `layers/` | The base layer, the tone layers, and the room layers under `rooms/` |
| `lore/` | The lore index, its chunks under `chunks/`, and the source documents the chunks are cut from |
| `src/` | The installer: the content read as `docs/format.md` states it, written as a brain file |
| `tests/` | The checks that hold the content to its format and the repository to its rules, and the installer's test |
| `docs/` | The canonical statements of the format, the boundary and the inventory |

At the root stand the Cargo manifest of the installer and the checks, the version, `testbed.toml` with its Korean ledger, `install.toml` (what installing the content on a fresh machine needs, read by the testbed's install verb), `rustfmt.toml`, the MIT `LICENSE` and `CLAUDE.md`.

## Documents

| Document | States |
|---|---|
| `docs/format.md` | The shape of the layers and their tone line, the lore index and its chunks, and the placeholders |
| `docs/boundary.md` | What the repository offers, how a deployment places it, the brain file the installer writes, which unit receives what, and where each reader of the source tree goes |
| `docs/inventory.md` | Every content file with its source and path, what was left out, and every placeholder that replaced a personal value |
| `docs/PERSONAL-DATA.md` | The values that never enter the repository, the two it allows, and the check that holds them |

## Version

- **The version is the set's,** read off the git tag `v<version>` when a release is made; no file in the
  repository holds it, and the package's version is a placeholder that never moves.
- **How a change counts toward the next release** is stated in `docs/boundary.md`, 「What this repository is」.

## Commands

```sh
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run --bin bmo-persona-install -- --out <file>
```

The last writes the brain file (`docs/boundary.md`, 「The brain file」); injecting it into a brain is the
operator's step.
