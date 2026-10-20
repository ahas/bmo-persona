# bmo-persona

The persona layers and the lore library of a chat agent, kept as files. The base layer is the persona the
agent's system prompt carries; tone layers and room layers are laid over it room by room; the lore library
is an index of chunks the agent is offered as reference. Personal values in the layers are placeholders a
deployment fills in from its own configuration. The repository runs nothing: its checks hold the content to
the format its readers rely on, and a deployment places it where those readers look.

## Layout

| Folder | Holds |
|---|---|
| `layers/` | The base layer, the tone layers, and the room layers under `rooms/` |
| `lore/` | The lore index, its chunks under `chunks/`, and the source documents the chunks are cut from |
| `tests/` | The checks that hold the content to its format and the repository to its rules |
| `docs/` | The canonical statements of the format, the boundary and the inventory |
| `.graph/` | The code dependency graph: one node per feature with its files and what it reads, written only by `bmo graph` |

At the root stand the Cargo manifest of the checks, the version, `testbed.toml` with its Korean ledger,
`rustfmt.toml`, the MIT `LICENSE` and `CLAUDE.md`.

## Documents

| Document | States |
|---|---|
| `docs/format.md` | The shape of the layers and their tone line, the lore index and its chunks, and the placeholders |
| `docs/boundary.md` | What the repository offers, how a deployment places it, which unit receives what, and where each reader of the source tree goes |
| `docs/inventory.md` | Every content file with its source and path, what was left out, and every placeholder that replaced a personal value |

## Version

- **One version for the repository,** in semver. `VERSION` holds it and the package's version equals it.
- **How a change moves it** is stated in `docs/boundary.md`, 「What this repository is」.

## Commands

```sh
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
