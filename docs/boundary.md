# Boundary

What this repository offers the other units, how each of them receives it, and where every reader of the
source tree's persona and lore goes.

## What this repository is

- **A data repository with one program:** the persona layers and the lore library, as files (`format.md`),
  and the installer that writes them as a brain file (「The brain file」). It runs no server, exposes no API,
  calls no unit and depends on no other repository.
- **No unit's code reads this repository or its paths.** What a unit needs of the content reaches it as a
  copy a deployment places in that unit's own tree, or through another unit's API.
- **The persona and the lore are bound for the chat agent's brain,** which serves them to their readers
  through its own API. They reach it as the brain file the installer writes and the operator injects. The
  repository knows the brain file's shape as the brain's boundary states it, and nothing else of the brain.
  Until the brain serves them to every reader, the copies below are how each unit receives the content.
- **The content is versioned** by `VERSION`, in semantic versioning. A change to anything `format.md` states
  is a major version; a change to the content alone is a minor one; a change to the installer or the checks
  alone is a patch.

## The brain file

The installer reads the content tree and writes a brain file: the SQLite database the brain's
`POST /brains/{brainId}/injections` takes, in the shape the brain's boundary states under 「Brain files」,
shape 1.

- **The command** is `bmo-persona-install --out <file>`, with `--root <dir>` for the content tree (the
  current directory where none is given) and `--name <name>` for the file's name (the package's name where
  none is given). From a checkout it runs as `cargo run --bin bmo-persona-install -- --out <file>`. A file
  standing at `--out` is replaced, and the installer prints how many layers and pieces it wrote.
- **Injecting the file is the operator's step:** `POST /brains/{brainId}/injections` on the brain, the file
  as the body (`application/vnd.sqlite3`). The installer calls no unit, and the repository holds no brain id.
- **The tables:** `brain_file` holds one row, shape 1, the moment of writing as `exported_at` in
  milliseconds since the epoch, and the name. `layers` and `memories` hold what follows. `subjects` and
  `memory_subjects` stand empty.
- **The layers, positioned from zero in this order:** `persona` for the base layer; `persona/tone/<tone>`
  for each tone layer, in the order of the tones' names; `persona/room/<slug>` for each room layer, in slug
  order; then the lore's layers in the index's order, `lore` for the entries above every section and
  `lore/<heading>` for each section. Each layer's description says what it is: a lore section's is its
  heading, and a room layer's names the tone its tone line chooses. A persona layer whose text is empty
  stands with no piece, and a section with no entry is no layer.
- **A section of the lore index** is a level-two heading. An entry belongs to the section whose heading
  stands last above it, and an entry above every such heading belongs to none. Sections with the same
  heading are one layer.
- **A persona piece** is one memory per layer file whose text is not empty: type `persona`, its layer's name as its key, position
  zero in its layer, and as its text the layer as its reader takes it (`format.md`), trimmed and, for a room
  layer, what follows its tone line. Its placeholders stay as they stand. Its labels are `source`, the
  file's path in the tree, and for a room layer with a tone line `tone`, the name the line gives. Its
  importance is 8.
- **A lore piece** is one memory per index entry: type `lore`, `lore/<slug>` as its key, the entry's
  position as its position, its section's layer, and as its text the entry's chunk, trimmed. Its labels are
  `source`, the chunk's path in the tree, `title`, the entry's title, and `scope`, the entry's scope tag
  where it has one. Its importance is 5. The source documents give no piece.
- **Every memory** is `active`, its own lineage with no successor, valid from the moment of writing, with
  no hit and no confirmation, strength 1, written by `bmo-persona-install`.
- **No memory is linked to a subject:** `format.md` states no form in which the lore names a person or a
  place for a machine to read.
- **Running the installer again gives the same layer names and keys,** so an injection of the new file
  reuses the layers by name, supersedes only the pieces whose text, type or layer changed, and adds nothing
  twice.

## How a deployment places the content

- **The persona layers go to the receiver's data tree,** under `prompts/persona/`: `layers/base.md`, each tone
  layer under its own name, and each room layer under `rooms/<chat id>.md`.
  - **Each room slug is mapped back to its room's chat id,** and **every placeholder is filled in,** from the
    deployment's own configuration. That configuration is kept outside every repository; this repository
    holds neither the values nor the mapping.
  - **After the filling, no placeholder is left,** and each layer is what its reader read before
    (`format.md`, 「Placeholders」).
- **The lore goes to the same data tree,** under `docs/lore/`, unchanged. The `bmo` tone layer tells the agent
  to find a chunk in `docs/lore/index.md`, so this is also the path the agent is told to read.
- **The copy is the deployment's step,** not any unit's code. This repository's testbed declaration
  declares no deployment of its own.

## Who receives what

| Unit | What it receives | From where | How |
|---|---|---|---|
| bmo-receiver | the base, tone and room layers | its own data tree, `prompts/persona/` | its room module reads the base layer for the chat line's system prompt, and a room's file, tone line first, for the tone and room layers of each turn (`format.md`) |
| bmo-receiver | notice of a changed layer or lore piece | its own data tree, `prompts/persona/` and `docs/lore/` | its reload verb watches both trees as documents, and names a changed lore index or chunk to the sessions that hold it |
| bmo-agency | the base persona | the receiver, with each turn | `POST /session/turn` carries it as `basePersona` in the turn's context; the agency holds no persona and no lore of its own |
| bmo-brain | the persona pieces and the lore pieces, as a brain file | the installer, run by the operator | `POST /brains/{brainId}/injections` with the file as the body (「The brain file」); the brain keeps each piece as a memory, body and all |
| bmo-gate | lore pieces for a room that reads the lore | bmo-brain | its `system-documents` gate offers the `lore` memories the brain keeps, by their layers, beside the system's own reference documents |
| bmo-web | the persona as a document root | the unit that serves it | the documents viewer lists the persona root as another unit's, unavailable until a unit serves it as whole documents; it reads no folder by path |

- **Another chat agent's persona** is kept by that agent's own persona repository. This repository does not
  know it, and it does not know this one.

## Open for the switch-over

- **The lore reaches the brain only by injection.** In the source tree, the fragment editors read the lore
  index and hand its entries to the fragment table, and the reference road offers the lore library to a room
  whose tone reads it. Neither has a home in the new units; the installer's brain file takes their place.
  Until the operator injects it, the brain holds no lore rows, and the gate offers no lore.
- **The web viewer expects the persona root from the agency,** but the agency holds no persona: the layers
  sit in the receiver's data tree. The switch-over names the unit that serves them as whole documents.

## Where the source tree's readers go

Every reader of the persona and lore in the source tree, one row each: the sixteen the first reader trace found,
then the thirteen this item's trace added. A reader of other prompts is listed with its unit too, so every member
of both traces stands here once.

| # | Reader in the source tree | What it read | Goes to | Receiving end |
|---|---|---|---|---|
| 1 | the bot's agent documents module: `agentFile`, `rulebook`, `chatSystemStatic`, `chatSystem` | agent documents, rulebooks, and the base persona composed into the chat system prompt | bmo-agency for the agent documents and rulebooks; bmo-receiver for the base persona | the receiver's room module reads the base layer from its data tree |
| 2 | the bot's persona module: `personaDir`, `loadPersona`, `withBasePersona` | the base layer | bmo-receiver | the room module's base persona, from its data tree |
| 3 | the bot's lore module: `loreIndexRelPath`, `loreChunksRelDir`, `loreLibrary`, `systemIndexRelPath` | the lore index and chunk paths | bmo-brain, through the installer's brain file | `POST /brains/{brainId}/injections` on bmo-brain, by the operator |
| 4 | the bot's lore module: `referenceWarmLines`, `peopleSearchWarmLine` | the reference gates' system lines over the lore | bmo-gate | the gate's own system prompts; the lore rows come from bmo-brain |
| 5 | the bot's co-worker roster | the co-worker definitions, not this content | bmo-agency | its own prompts |
| 6 | the voice tool's lines module | the voice agent's instructions, not this content | bmo-voice | its own prompts |
| 7 | the bot's agent documents module: `deployAgents` | the composed system prompts, base persona included, written to the deploy tree | bmo-agency for the system prompts; bmo-receiver for the base persona | the receiver hands the base persona in with each turn as `basePersona` |
| 8 | the agent library's prompt files: `systemPromptFile` | the composed system prompt, not this content directly | bmo-agency | its own launch files |
| 9 | the agent library's run log: `readSystemPrompt`, `launched` | the composed system prompt, as above | bmo-agency | as above |
| 10 | the agent library's dsh harness | the composed system prompt, as above | bmo-agency | as above |
| 11 | the agent library's Claude Code harness | the composed system prompt, as above | bmo-agency | as above |
| 12 | the agent library's Codex harness | the composed system prompt, as above | bmo-agency | as above |
| 13 | the bot's fragments module: `fragmentLibraries` | the lore and system document libraries | bmo-brain, through the installer's brain file | `POST /brains/{brainId}/injections` on bmo-brain, by the operator |
| 14 | the bot's lore module: `treePieces` | the lore index, as fragment entries with chunk paths | bmo-brain, through the installer's brain file | as above |
| 15 | the gate's live reference inputs: `bmoInputs` | lore and reference candidates, through a verb | bmo-gate | lore rows through bmo-brain |
| 16 | the console's documents utility: its document filter | the documents tree, leaving the lore out of the top-level documents | bmo-web | the persona root, listed as another unit's (「Who receives what」) |
| 17 | the server's chat room module: `persona_file`, `persona_dir`, `with_base_persona`, `room_persona` | the base layer, a room's file by chat id, and the tone layer its tone line names | bmo-receiver | its room module, from its data tree |
| 18 | the server's reload verb: `DOCUMENT_DIRS`, and its lore index and reference index checks | the persona and lore trees as watched documents | bmo-receiver | its reload verb, over its data tree |
| 19 | the server's reference road: `bestRoad`, `shipped` | the system reference index, and the lore index for a room whose tone reads it | bmo-brain, through the installer's brain file, with bmo-gate choosing what a room is offered | `POST /brains/{brainId}/injections` on bmo-brain, by the operator |
| 20 | the server's fragment editors' libraries: `tree_pieces` | the lore index, as fragment entries with chunk paths | bmo-brain, through the installer's brain file | as above |
| 21 | the server's chat window module: its person-book owner test | the lore index path, as a test value | bmo-receiver | its own window test |
| 22 | the server's fragment verb integration test | fixture lore index and chunk files | bmo-brain | its own fragment tests, with its own fixtures |
| 23 | the server's reference bench verbs integration test | a fixture room layer | bmo-gate | its own reference bench, with its own fixtures |
| 24 | the server's record verbs integration test | a fixture lore document, as a pending reload notice | bmo-receiver | its own reload tests |
| 25 | the testbed suite's selection test | the persona and lore paths, as test values for change selection | bmo-testbed | its own selection test |
| 26 | the harness machine's companion prompt | a companion's own persona file in its prompt folder, not this content | bmo-agency | its own prompt folders |
| 27 | the language crate's companions: `persona_file` | a companion's persona file name, as a field | bmo-web | its own language pack |
| 28 | the policy `chat-line.toml`, its `persona` section | the headers the base, tone and room layers sit under | bmo-receiver | its own policy |
| 29 | the policy `companions.toml`: its persona piece and `persona_most` | a companion's persona length limit, not this content | bmo-agency | its own policy |
