# Recovery and native architecture

Input: `java/TheLegendOfZeldaMobile-beta4.jar`, SHA-256
`5a9183745571117f850a51414ededee4afe3b99e3dc014a7ece0cae584bc199a`.
The manifest calls it version 4.12, MIDP 2.0 / CLDC 1.1. The archive contains
17 classes and 757 original resource files.

## How the game runs

The executable game is maintained directly in `src/game/`. These Rust files are
its source of truth: edit them and build with Cargo. There is no Java-to-Rust
compiler, build-time translation step, operand stack, or bytecode program counter.
Neither the original JAR nor a Java installation is needed to build or play.

The first restoration mechanically translated 103 methods (73,228 JVM
instructions) into 8,391 control-flow blocks. That representation has been
retired. `java/reference/translation.json` records its historical coverage; it is
not an input to the native game. CFR's 19,146 lines under
`java/reference/decompiled/` remain an inspection aid. The decompiler could not
completely reconstruct some large
methods, so the native behavior is checked against the unmodified JAR rather
than relying on that Java as compilable source.

Gameplay functions have typed arguments and descriptive names, such as
`world::load_map`, `item::collect`, and `player::encode_save`. Field accesses use
named keys such as `fields::player::HEALTH`. Branches and loops are native Rust;
complex shared exits use labeled blocks to keep their side effects in one place.
A few large routines align these exits and use `rustfmt::skip` to avoid excessive
indentation. Their branch logic is still ordinary, directly editable Rust.
Fields whose meaning has not been established retain `STATE_*` names with their
original reference identifier documented in `fields.rs`, rather than invented
explanations.

`src/runtime.rs` supplies object identity, arrays, vectors, image handles and the
platform services needed by this game. The tagged values and object arena retain
reference and save semantics; collection runs only between complete game calls,
when native roots are known. Legacy class/field/method signatures are isolated
in `src/game/compat.rs` for the service boundary and reference/audit tools. They
do not contain executable game logic. Typed service wrappers live in `host.rs`.

Unknown services fail explicitly. Images, clipping, translations, anchors,
transforms, fonts, streams, original big-endian record payloads, and Java's
48-bit PRNG are implemented in Rust. Timer callbacks preserve the original
70ms/1000ms periods. The sound worker's finite polling work runs cooperatively;
actual MIDI/WAV rendering runs on SDL's audio thread.

## Native module guide and recovered class roles

All gameplay paths below are relative to `src/game/`. `canvas.rs` and `world.rs`
are entry points that group their logic into smaller files in `canvas/` and
`world/` (controls, saves, inventory, collision, rooms, combat and rendering).

| Original class | Native module | Responsibility |
|---|---|---|
| Zelda | `lifecycle.rs` | Application lifecycle and softkey commands |
| k | `canvas.rs` | Main game canvas, menus, input, progression and save slots |
| j | `world.rs` | Map loading, collision, transitions and entity updates |
| h | `player.rs`, `player_save.rs` | Player drawing, inventory/progression state, Triforce counting and the 172-byte save format |
| l | `enemy.rs` | Enemy and boss behavior and drawing |
| c | `item.rs` | Collectibles, equipment, shops and pickup effects |
| n | `map_object.rs` | Scenery, triggers and scripted interactions |
| o | `resources.rs` | Explicit sprite catalogs, shared sheets and cache lifetimes |
| p | `projectile.rs` | Projectiles and visual effects |
| f | `drawable.rs` | Common drawable position and bounds |
| a / m | `second_timer.rs`, `frame_timer.rs` | One-second simulation flags / 70ms repaint and command callbacks |
| b | `sound.rs` | Music selection, effects, volume and priority; `src/audio.rs` performs synthesis |
| d / e / g / i | `motion.rs`, `portal.rs`, `trail_point.rs`, `room_history.rs` | Small motion, portal, hookshot-trail and room-history records |

For example, change pickup rules in `item::collect`, movement checks in
`world/collision.rs` (`world::can_player_move`), sprite choices in `resources.rs`, or save layout in
`player_save.rs`. Changes to these files survive reference recovery.

## XMOB map format

All 505 files use the same layout, without compression or trailing data:

1. ASCII `XMOB`; width byte; height byte; tileset byte.
2. `width * height` tile bytes, row major. This archive always uses 16×11 tiles.
   The low six bits select a tile; upper bits retain the original collision flags.
3. Count byte followed by 6-byte portal/transition records.
4. Count byte followed by 3-byte enemy records: tile X, tile Y, type.
5. Count byte followed by 4-byte item records: tile X, tile Y, type, persistence ID.
6. Count byte followed by 10-byte object/trigger records, including packed flags.
7. Count byte followed by strings, each encoded as length byte + Latin-1 bytes.

Tile dimensions are 16×16 pixels. The scene is 256×176, with the original 32-pixel
HUD offset. The preserved 240×320 canvas therefore includes the original unused
space below the scene. There are 25 used tilesets, 4,973 transitions, 1,285 enemy
placements, 171 items, 10,631 objects and 63 text entries across the map variants.

## Saves and desktop adaptations

The three original stores (`ZeldaSave`, `ZeldaSave2`, `ZeldaSave3`) retain the
172-byte player payload encoded directly by `player_save.rs`. Integers
are big endian; booleans and byte fields preserve their original encoding.
`ZeldaSettings` remains separate. Native files wrap each payload in `ZMR1`, a
four-byte big-endian payload length, then the original bytes. Writes replace the
old file atomically after flushing the temporary file.

Native bindings live in a separate `controls.json`. The first-run forced phone
key-configuration selection is skipped because native keyboard/controller
bindings are already present. Secondary action backs out of native-adapted menus.
Focus loss and the native controls screen pause simulation/audio and clear held
inputs. Closing the native window saves an active character and the settings.

The original beta's gameplay and data are preserved, including its test-room
sign, test maps, spelling, original credits, and unfinished/quirky content.
GeneralUser GS preserves the MIDI sequences with consistent offline instruments;
its timbre can differ from an original phone synthesizer.
