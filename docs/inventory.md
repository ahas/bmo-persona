# Inventory

What this repository holds, where each file comes from, and what was changed on the way in.

## Source

- **The source tree** is the single repository this unit is split out of, at commit `3e4c032da`. Every
  file below is listed by `git ls-tree -r --name-only 3e4c032da prompts/persona docs/lore`, which
  answers 86 paths.
- **The split decision** is archive document #301: it places this unit among the data repositories,
  a backup of the persona and lore documents with no process, no API and no code that references it.
- **The working branch of the earlier restructuring** (`work/1703`) holds no difference from the
  source commit under either folder (`git diff --stat 3e4c032da work/1703 -- prompts/persona docs/lore`
  answers nothing).

## Counts

| What | Count | Source |
|---|---|---|
| Content files | 86 | `git ls-tree` at the source commit, as above |
| of which persona layers (`prompts/persona/` → `layers/`) | 7 | the same listing |
| of which lore (`docs/lore/` → `lore/`) | 79 | the same listing: 5 top-level files and 74 chunks |
| Crates | 0 | #301: a data repository has no code |
| Packages | 0 | #301, as above |
| Policy files | 0 | the reader trace of this item: the `[persona]` section of `chat-line.toml` (layer headers) belongs to the chat receiver, and `companions.toml` (the companion persona piece and its length limit) to the agency |
| Tests | 0 | the reader trace of this item: every old test touching these files tests a reader, and each reader goes to the unit that owns it (`boundary.md`); this repository's own tests in `tests/` guard the format those readers rely on (`format.md`) |
| Service units | 0 | #301, as above |

## Persona layers

`prompts/persona/` → `layers/`, 7 files. The room layers lose the room id they were named by and take
a neutral slug instead, assigned in the sort order of the source file names. The slug's room is the
deployment's to know (「Placeholders」 below).

| Old path | New path | Replacements |
|---|---|---|
| `prompts/persona/base.md` | `layers/base.md` | yes |
| `prompts/persona/bmo.md` | `layers/bmo.md` | yes |
| `prompts/persona/cynical.md` | `layers/cynical.md` | yes |
| `prompts/persona/rooms/<room id>.md`, first in sort order | `layers/rooms/room-a.md` | yes |
| `prompts/persona/rooms/<room id>.md`, second in sort order | `layers/rooms/room-b.md` | yes |
| `prompts/persona/rooms/<room id>.md`, third in sort order | `layers/rooms/room-c.md` | yes |
| `prompts/persona/rooms/<room id>.md`, fourth in sort order | `layers/rooms/room-d.md` | file name only |

## Lore, top level

`docs/lore/` → `lore/`, 5 files, copied unchanged.

| Old path | New path |
|---|---|
| `docs/lore/bmo-character.md` | `lore/bmo-character.md` |
| `docs/lore/bmo-history.md` | `lore/bmo-history.md` |
| `docs/lore/index.md` | `lore/index.md` |
| `docs/lore/relations.md` | `lore/relations.md` |
| `docs/lore/work.md` | `lore/work.md` |

## Lore chunks

`docs/lore/chunks/` → `lore/chunks/`, 74 files, copied unchanged.

| Old path | New path |
|---|---|
| `docs/lore/chunks/after-the-war.md` | `lore/chunks/after-the-war.md` |
| `docs/lore/chunks/air.md` | `lore/chunks/air.md` |
| `docs/lore/chunks/amo.md` | `lore/chunks/amo.md` |
| `docs/lore/chunks/bmo-allmo.md` | `lore/chunks/bmo-allmo.md` |
| `docs/lore/chunks/bmo-amo-confrontation.md` | `lore/chunks/bmo-amo-confrontation.md` |
| `docs/lore/chunks/bmo-birthday-and-false-moe.md` | `lore/chunks/bmo-birthday-and-false-moe.md` |
| `docs/lore/chunks/bmo-child-and-logic.md` | `lore/chunks/bmo-child-and-logic.md` |
| `docs/lore/chunks/bmo-detective-play.md` | `lore/chunks/bmo-detective-play.md` |
| `docs/lore/chunks/bmo-drift-arrival.md` | `lore/chunks/bmo-drift-arrival.md` |
| `docs/lore/chunks/bmo-drift-hugo.md` | `lore/chunks/bmo-drift-hugo.md` |
| `docs/lore/chunks/bmo-eagle-bubble-air.md` | `lore/chunks/bmo-eagle-bubble-air.md` |
| `docs/lore/chunks/bmo-emotions-in-machine-words.md` | `lore/chunks/bmo-emotions-in-machine-words.md` |
| `docs/lore/chunks/bmo-factory-return.md` | `lore/chunks/bmo-factory-return.md` |
| `docs/lore/chunks/bmo-fears.md` | `lore/chunks/bmo-fears.md` |
| `docs/lore/chunks/bmo-football-mirror.md` | `lore/chunks/bmo-football-mirror.md` |
| `docs/lore/chunks/bmo-football-swap.md` | `lore/chunks/bmo-football-swap.md` |
| `docs/lore/chunks/bmo-game-portal.md` | `lore/chunks/bmo-game-portal.md` |
| `docs/lore/chunks/bmo-gender.md` | `lore/chunks/bmo-gender.md` |
| `docs/lore/chunks/bmo-gum-war-song.md` | `lore/chunks/bmo-gum-war-song.md` |
| `docs/lore/chunks/bmo-identity-body.md` | `lore/chunks/bmo-identity-body.md` |
| `docs/lore/chunks/bmo-king-of-ooo.md` | `lore/chunks/bmo-king-of-ooo.md` |
| `docs/lore/chunks/bmo-likes-and-helpfulness.md` | `lore/chunks/bmo-likes-and-helpfulness.md` |
| `docs/lore/chunks/bmo-living-little-boy.md` | `lore/chunks/bmo-living-little-boy.md` |
| `docs/lore/chunks/bmo-meeting-moe.md` | `lore/chunks/bmo-meeting-moe.md` |
| `docs/lore/chunks/bmo-misc-episodes.md` | `lore/chunks/bmo-misc-episodes.md` |
| `docs/lore/chunks/bmo-moe-backup-disk.md` | `lore/chunks/bmo-moe-backup-disk.md` |
| `docs/lore/chunks/bmo-open-questions-history.md` | `lore/chunks/bmo-open-questions-history.md` |
| `docs/lore/chunks/bmo-open-questions-self.md` | `lore/chunks/bmo-open-questions-self.md` |
| `docs/lore/chunks/bmo-people-sense.md` | `lore/chunks/bmo-people-sense.md` |
| `docs/lore/chunks/bmo-pretend-is-real.md` | `lore/chunks/bmo-pretend-is-real.md` |
| `docs/lore/chunks/bmo-songs-and-catchphrases.md` | `lore/chunks/bmo-songs-and-catchphrases.md` |
| `docs/lore/chunks/bmo-speech-patterns.md` | `lore/chunks/bmo-speech-patterns.md` |
| `docs/lore/chunks/bmo-stubborn-and-fierce.md` | `lore/chunks/bmo-stubborn-and-fierce.md` |
| `docs/lore/chunks/bmo-treehouse-life.md` | `lore/chunks/bmo-treehouse-life.md` |
| `docs/lore/chunks/candy-kingdom.md` | `lore/chunks/candy-kingdom.md` |
| `docs/lore/chunks/candy-people.md` | `lore/chunks/candy-people.md` |
| `docs/lore/chunks/dead-worlds.md` | `lore/chunks/dead-worlds.md` |
| `docs/lore/chunks/death-in-ooo.md` | `lore/chunks/death-in-ooo.md` |
| `docs/lore/chunks/elemental-avatars.md` | `lore/chunks/elemental-avatars.md` |
| `docs/lore/chunks/finn.md` | `lore/chunks/finn.md` |
| `docs/lore/chunks/fire-kingdom.md` | `lore/chunks/fire-kingdom.md` |
| `docs/lore/chunks/football.md` | `lore/chunks/football.md` |
| `docs/lore/chunks/hugo-and-mr-m.md` | `lore/chunks/hugo-and-mr-m.md` |
| `docs/lore/chunks/ice-king.md` | `lore/chunks/ice-king.md` |
| `docs/lore/chunks/ice-kingdom.md` | `lore/chunks/ice-kingdom.md` |
| `docs/lore/chunks/jake-and-lady-children.md` | `lore/chunks/jake-and-lady-children.md` |
| `docs/lore/chunks/jake.md` | `lore/chunks/jake.md` |
| `docs/lore/chunks/kingdoms-and-rule.md` | `lore/chunks/kingdoms-and-rule.md` |
| `docs/lore/chunks/lady-rainicorn.md` | `lore/chunks/lady-rainicorn.md` |
| `docs/lore/chunks/land-of-the-dead.md` | `lore/chunks/land-of-the-dead.md` |
| `docs/lore/chunks/life-in-ooo.md` | `lore/chunks/life-in-ooo.md` |
| `docs/lore/chunks/lorraine.md` | `lore/chunks/lorraine.md` |
| `docs/lore/chunks/magic-and-technology.md` | `lore/chunks/magic-and-technology.md` |
| `docs/lore/chunks/marceline.md` | `lore/chunks/marceline.md` |
| `docs/lore/chunks/mo-brothers-and-allmo.md` | `lore/chunks/mo-brothers-and-allmo.md` |
| `docs/lore/chunks/moe-and-origin.md` | `lore/chunks/moe-and-origin.md` |
| `docs/lore/chunks/moe.md` | `lore/chunks/moe.md` |
| `docs/lore/chunks/money-language-belief.md` | `lore/chunks/money-language-belief.md` |
| `docs/lore/chunks/mushroom-bomb.md` | `lore/chunks/mushroom-bomb.md` |
| `docs/lore/chunks/mushroom-war.md` | `lore/chunks/mushroom-war.md` |
| `docs/lore/chunks/neptr.md` | `lore/chunks/neptr.md` |
| `docs/lore/chunks/olive.md` | `lore/chunks/olive.md` |
| `docs/lore/chunks/ooo-geography.md` | `lore/chunks/ooo-geography.md` |
| `docs/lore/chunks/ooo-regions.md` | `lore/chunks/ooo-regions.md` |
| `docs/lore/chunks/open-questions.md` | `lore/chunks/open-questions.md` |
| `docs/lore/chunks/other-dimensions.md` | `lore/chunks/other-dimensions.md` |
| `docs/lore/chunks/princess-bubblegum.md` | `lore/chunks/princess-bubblegum.md` |
| `docs/lore/chunks/shelby-and-housemates.md` | `lore/chunks/shelby-and-housemates.md` |
| `docs/lore/chunks/sparkle.md` | `lore/chunks/sparkle.md` |
| `docs/lore/chunks/susan-strong.md` | `lore/chunks/susan-strong.md` |
| `docs/lore/chunks/time-and-wishes.md` | `lore/chunks/time-and-wishes.md` |
| `docs/lore/chunks/wizard-city.md` | `lore/chunks/wizard-city.md` |
| `docs/lore/chunks/world-threats.md` | `lore/chunks/world-threats.md` |
| `docs/lore/chunks/y5.md` | `lore/chunks/y5.md` |

## Not taken

Six files beside these are left where they are. `<agent>` stands for another chat agent's folder name.

| Path | Why |
|---|---|
| `bot/bot/persona.ts` | code that reads the persona layers; a reader goes with the unit that owns it (`boundary.md`) |
| `bot/bot/lore.ts` | code that reads the lore; as above |
| `prompts/agents/<agent>-chat/persona.md` | the persona of another chat agent, kept by that agent's own persona repository |
| `prompts/agents/<agent>-chat/picture.jpg` | that agent's picture, kept with its registration, not persona or lore text |
| `prompts/agents/<agent>/handoff.md` | that agent's working document, kept by its own persona repository |
| `prompts/agents/<agent>/own-rules.md` | that agent's own rules, kept by its own persona repository |

## Placeholders

- **A personal value in a persona layer is replaced by a placeholder:** two opening braces, a key and
  two closing braces, as in `{{owner.name}}`. A key is one or more lowercase words of letters, digits
  and hyphens, joined by dots. No content file used double braces before the replacement, so every
  pair in the content is a placeholder.
- **A placeholder stands for the exact text it replaced,** so a file with every placeholder filled in
  is byte for byte the file it was copied from.
- **The values are never written into this repository.** A deployment keeps them in its own
  configuration and fills them in when it puts the layers in front of their reader (`boundary.md`). The
  same configuration maps each room slug to the room it stands for.
- **Two keys hold across every file:**

| Key | Stands for |
|---|---|
| `owner.name` | the name the owner is called by |
| `owner.id` | the owner's chat platform account id |

- **The other keys belong to the room layer they appear in:** the same key in two room layers stands
  for two different values.

| Key | Stands for |
|---|---|
| `room.title` | the room's name, the layer's heading |
| `person-<n>.name` | the real name of the room's n-th listed person after the owner |
| `person-<n>.id` | that person's chat platform account id |
| `person-<n>.called` | what that person is called, with any alternative named beside it |
| `person-<n>.age` | that person's age relative to someone else in the room |
| `person-<n>.relation` | that person's relationship to the owner and place in the room |
| `bot-<n>.name` | the name the bot sharing the room goes by |
| `bot-<n>.id` | the chat platform account id of another bot sharing the room |
| `bot-<n>.handle` | that bot's chat platform handle |

## Replacements

By file, line and class. Each layer was read whole against the persona sweep of this item and a
pattern sweep over every file: lines not named here hold no personal value. The lore, swept the same
way over all 79 files, holds none and is copied unchanged.

| File | Line | Placeholders |
|---|---|---|
| `layers/base.md` | 11 | `owner.name` |
| `layers/base.md` | 19 | `owner.name` |
| `layers/base.md` | 24 | `owner.name` |
| `layers/base.md` | 31 | `owner.name` twice, `owner.id` |
| `layers/base.md` | 33 | `owner.name` |
| `layers/base.md` | 50 | `owner.name` |
| `layers/base.md` | 55 | `owner.name` |
| `layers/base.md` | 64 | `owner.name` |
| `layers/base.md` | 69 | `owner.name` |
| `layers/bmo.md` | 10 | `owner.name` |
| `layers/bmo.md` | 12 | `owner.name` |
| `layers/bmo.md` | 34 | `owner.name` |
| `layers/cynical.md` | 5 | `owner.name` |
| `layers/cynical.md` | 6 | `owner.name` |
| `layers/rooms/room-a.md` | 3 | `room.title` |
| `layers/rooms/room-a.md` | 10 | `owner.name` twice, `owner.id` |
| `layers/rooms/room-a.md` | 11 | `person-1.name`, `person-1.id`, `person-1.called`, `owner.name` |
| `layers/rooms/room-a.md` | 12 | `person-2.name`, `person-2.id`, `person-2.called`, `owner.name` |
| `layers/rooms/room-a.md` | 13 | `person-3.name`, `person-3.id`, `person-3.called`, `owner.name` |
| `layers/rooms/room-b.md` | 3 | `room.title` |
| `layers/rooms/room-b.md` | 10 | `owner.name` twice, `owner.id` |
| `layers/rooms/room-b.md` | 11 | `person-1.name`, `person-1.id`, `person-1.called`, `person-1.relation` |
| `layers/rooms/room-b.md` | 13 | `person-2.name`, `person-2.id`, `person-2.called`, `owner.name`, `person-2.age` |
| `layers/rooms/room-b.md` | 14 | `person-3.name`, `person-3.id`, `person-3.called`, `owner.name`, `person-3.age` |
| `layers/rooms/room-b.md` | 15 | `person-4.name`, `person-4.id`, `person-4.called`, `owner.name`, `person-4.age` |
| `layers/rooms/room-b.md` | 16 | `person-5.name`, `person-5.id`, `person-5.called`, `owner.name`, `person-5.age` |
| `layers/rooms/room-b.md` | 17 | `bot-1.name` twice, `bot-1.id` |
| `layers/rooms/room-b.md` | 19 | `bot-1.name` |
| `layers/rooms/room-b.md` | 21 | `bot-1.name`, `bot-1.handle` |
| `layers/rooms/room-b.md` | 22 | `bot-1.name` twice |
| `layers/rooms/room-b.md` | 24 | `owner.name`, `bot-1.name` twice |
| `layers/rooms/room-b.md` | 25 | `bot-1.name` |
| `layers/rooms/room-b.md` | 27 | `bot-1.name` three times |
| `layers/rooms/room-b.md` | 29 | `bot-1.name` |
| `layers/rooms/room-b.md` | 30 | `bot-1.name` twice |
| `layers/rooms/room-b.md` | 31 | `bot-1.handle` |
| `layers/rooms/room-c.md` | 3 | `room.title` |
| `layers/rooms/room-c.md` | 10 | `owner.name` twice, `owner.id` |
| `layers/rooms/room-c.md` | 11 | `person-1.name`, `person-1.id`, `person-1.called` |
| `layers/rooms/room-d.md` | none | its room id was only in its file name |

- **What stays:** the chat platform's name and the names of the agents, which are the persona's own
  registration data. A bot sharing a room goes by a placeholder like its people, so neither its name nor
  its handle is written here. Fictional characters of the lore are content, not people.
- **Outside the content,** in code, tests, `README.md`, `CLAUDE.md` and `docs/`, no agent is named and
  no personal value appears.
