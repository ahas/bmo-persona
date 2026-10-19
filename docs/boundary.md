# Boundary

What this repository offers the other units, how each of them receives it, and where every reader of the
source tree's persona and lore goes.

## What this repository is

- **A data repository:** the persona layers and the lore library, as files (`format.md`). It runs no
  server, exposes no API, calls no unit and depends on no other repository.
- **No unit's code reads this repository or its paths.** What a unit needs of the content reaches it as a
  copy a deployment places in that unit's own tree, or through another unit's API.
- **The content is versioned** by `VERSION`, in semantic versioning. A change to anything `format.md` states
  is a major version; a change to the content alone is a minor one.

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
| bmo-brain | the lore entries, as a shipped kind | the unit that ships the lore | `POST /libraries/reconcile` with `kind`, `label` and `entries`, each entry `slug`, `title`, `path`, `position`, `scope` or null, `items`, `writtenOn`, `confirmedOn` and `bodyHash`, read off `docs/lore/index.md` and its chunks; the brain keeps the rows, never the bodies |
| bmo-gate | lore pieces for a room that reads the lore | bmo-brain | its `system-documents` gate offers the lore rows the brain keeps beside the system's own reference documents |
| bmo-web | the persona as a document root | the unit that serves it | the documents viewer lists the persona root as another unit's, unavailable until a unit serves it as whole documents; it reads no folder by path |

- **Another chat agent's persona** is kept by that agent's own persona repository. This repository does not
  know it, and it does not know this one.

## Open for the switch-over

- **No unit is named to ship the lore yet.** In the source tree, the fragment editors read the lore index and
  hand its entries to the fragment table, and the reference road offers the lore library to a room whose tone
  reads it. Neither has a home in the new units. The switch-over names the unit that reads `docs/lore/` in its
  tree and calls `POST /libraries/reconcile` with it. Until then, the brain holds no lore rows, and the gate
  offers no lore.
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
| 3 | the bot's lore module: `loreIndexRelPath`, `loreChunksRelDir`, `loreLibrary`, `systemIndexRelPath` | the lore index and chunk paths | the unit that ships the lore, not yet named | `POST /libraries/reconcile` on bmo-brain |
| 4 | the bot's lore module: `referenceWarmLines`, `peopleSearchWarmLine` | the reference gates' system lines over the lore | bmo-gate | the gate's own system prompts; the lore rows come from bmo-brain |
| 5 | the bot's co-worker roster | the co-worker definitions, not this content | bmo-agency | its own prompts |
| 6 | the voice tool's lines module | the voice agent's instructions, not this content | bmo-voice | its own prompts |
| 7 | the bot's agent documents module: `deployAgents` | the composed system prompts, base persona included, written to the deploy tree | bmo-agency for the system prompts; bmo-receiver for the base persona | the receiver hands the base persona in with each turn as `basePersona` |
| 8 | the agent library's prompt files: `systemPromptFile` | the composed system prompt, not this content directly | bmo-agency | its own launch files |
| 9 | the agent library's run log: `readSystemPrompt`, `launched` | the composed system prompt, as above | bmo-agency | as above |
| 10 | the agent library's dsh harness | the composed system prompt, as above | bmo-agency | as above |
| 11 | the agent library's Claude Code harness | the composed system prompt, as above | bmo-agency | as above |
| 12 | the agent library's Codex harness | the composed system prompt, as above | bmo-agency | as above |
| 13 | the bot's fragments module: `fragmentLibraries` | the lore and system document libraries | the unit that ships the lore, not yet named | `POST /libraries/reconcile` on bmo-brain |
| 14 | the bot's lore module: `treePieces` | the lore index, as fragment entries with chunk paths | the unit that ships the lore, not yet named | as above |
| 15 | the gate's live reference inputs: `bmoInputs` | lore and reference candidates, through a verb | bmo-gate | lore rows through bmo-brain |
| 16 | the console's documents utility: its document filter | the documents tree, leaving the lore out of the top-level documents | bmo-web | the persona root, listed as another unit's (「Who receives what」) |
| 17 | the server's chat room module: `persona_file`, `persona_dir`, `with_base_persona`, `room_persona` | the base layer, a room's file by chat id, and the tone layer its tone line names | bmo-receiver | its room module, from its data tree |
| 18 | the server's reload verb: `DOCUMENT_DIRS`, and its lore index and reference index checks | the persona and lore trees as watched documents | bmo-receiver | its reload verb, over its data tree |
| 19 | the server's reference road: `bestRoad`, `shipped` | the system reference index, and the lore index for a room whose tone reads it | the unit that ships the lore, not yet named, with bmo-gate choosing what a room is offered | `POST /libraries/reconcile` on bmo-brain |
| 20 | the server's fragment editors' libraries: `tree_pieces` | the lore index, as fragment entries with chunk paths | the unit that ships the lore, not yet named | as above |
| 21 | the server's chat window module: its person-book owner test | the lore index path, as a test value | bmo-receiver | its own window test |
| 22 | the server's fragment verb integration test | fixture lore index and chunk files | bmo-brain | its own fragment tests, with its own fixtures |
| 23 | the server's reference bench verbs integration test | a fixture room layer | bmo-gate | its own reference bench, with its own fixtures |
| 24 | the server's record verbs integration test | a fixture lore document, as a pending reload notice | bmo-receiver | its own reload tests |
| 25 | the testbed suite's selection test | the persona and lore paths, as test values for change selection | bmo-testbed | its own selection test |
| 26 | the harness machine's companion prompt | a companion's own persona file in its prompt folder, not this content | bmo-agency | its own prompt folders |
| 27 | the language crate's companions: `persona_file` | a companion's persona file name, as a field | bmo-web | its own language pack |
| 28 | the policy `chat-line.toml`, its `persona` section | the headers the base, tone and room layers sit under | bmo-receiver | its own policy |
| 29 | the policy `companions.toml`: its persona piece and `persona_most` | a companion's persona length limit, not this content | bmo-agency | its own policy |
