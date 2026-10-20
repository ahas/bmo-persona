# Format

The shape the content's readers rely on. The repository's tests hold the content to it, so a reader that
keeps to this document reads the content the way the source tree's readers did.

## Persona layers

`layers/` holds three kinds of layer. Each file is markdown, and a reader takes its text trimmed of
surrounding white space; a missing file reads as empty.

- **The base layer, `layers/base.md`:** the base persona. A chat agent's system prompt carries it after the
  chat line's own prompt, under the persona header its reader keeps; with an empty base layer the prompt
  carries none.
- **The tone layers, one file per tone, named by the tone:** `layers/bmo.md` and `layers/cynical.md`. A tone
  layer is laid over the base layer for a room that takes it, under the tone header its reader keeps.
- **The room layers, `layers/rooms/<slug>.md`:** what one room is and who is in it, laid over the tone layer
  under the room header its reader keeps. A room with no file has no room layer.

### A room layer's tone line

- **A room layer may open with a tone line:** `tone:`, optional spaces or tabs, the tone's name as one run of
  characters with no white space, optional spaces or tabs, then the end of the line.
- **The name chooses the tone layer** of the same name; `none` chooses no tone layer.
- **The rest of the file after the tone line, trimmed, is the room layer.** A file with no tone line is all
  room layer, and which tone the room takes is then its reader's default.
- **Every tone line in this repository names a tone layer that exists, or `none`.**

## Lore

`lore/` holds the lore library: its index, its chunks, and the source documents the chunks are cut from.

- **`lore/index.md`** is the index a reader offers the lore by.
- **`lore/chunks/<slug>.md`** is the body of the index entry with that slug.
- **`lore/bmo-character.md`, `lore/bmo-history.md`, `lore/relations.md` and `lore/work.md`** are the source
  documents. No index entry points at them, and no reader offers them as pieces.

### An index entry

- **One entry is one line:** a dash and a space, the slug, optionally white space and a scope tag in square
  brackets, then white space, an em dash, white space and the title, which runs to the end of the line.
- **The slug** is one or more lowercase ASCII letters, digits and hyphens.
- **The scope tag** is taken as written, brackets included; an entry with none has no scope.
- **A line of any other shape is not an entry:** headings, prose and blank lines are skipped.
- **The first line with a slug wins;** a later line with the same slug is ignored.
- **An entry's position** is its place among the entries in the order they first appear, counting from zero.
- **Every entry has its chunk, and every chunk has its entry.**

## Placeholders

A personal value in a persona layer is replaced by a placeholder; `inventory.md` 「Placeholders」 is its whole
statement, and in short:

- **A placeholder is two opening braces, a key and two closing braces.** A key is one or more words of lowercase
  ASCII letters, digits and hyphens, joined by dots.
- **Only the documented keys appear:** `owner.name` and `owner.id` in any layer, and in a room layer
  `room.title`, `person-<n>.name`, `person-<n>.id`, `person-<n>.called`, `person-<n>.age`,
  `person-<n>.relation`, `bot-<n>.name`, `bot-<n>.id` and `bot-<n>.handle`, where `<n>` is a number
  from one.
- **A placeholder stands for the exact text it replaced.** Filling every placeholder in a layer with its value
  gives back the layer as its reader read it before, byte for byte.
- **A reader never sees a placeholder.** The deployment fills them in before the layers reach their reader
  (`boundary.md`).
- **The lore holds no placeholders.**
