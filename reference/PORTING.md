# Recovery and native architecture

Input: `TheLegendOfZeldaMobile-beta4.jar`, SHA-256
`5a9183745571117f850a51414ededee4afe3b99e3dc014a7ece0cae584bc199a`.
The manifest calls it version 4.12, MIDP 2.0 / CLDC 1.1. The archive contains
17 classes and 757 original resource files.

## How the game runs

CFR recovered 19,146 lines of readable Java under `reference/java`. Obfuscation
produces invalid variables and ambiguous reconstructed control flow in several
large methods. This readable output is an inspection aid, rather than the source
of executable behavior.

`scripts/translate.py` reads the original class files and translates all 103
executable methods (73,228 JVM instructions) into 8,391 Rust control-flow blocks.
Each method is a Rust function in `src/generated`; integer/float operations,
branches, method calls, inherited fields, and exception handlers become Rust code.
`reference/translation.json` records method signatures and coverage.

The result is compiled ahead of time by Rust/LLVM. The running app loads **no Java
classes or bytecode** and contains no JVM or general-purpose bytecode interpreter.
The tagged values and object arena in `src/runtime.rs` preserve the original
program's field, array, vector, and reference semantics. Collection runs only
between complete game calls, when native roots are known.

Native host services implement the exact API subset this game uses. Unknown
services fail explicitly. Images, clipping, translations, anchors, transforms,
fonts, streams, original big-endian record payloads, and Java's 48-bit PRNG are
implemented in Rust. Timer callbacks preserve the original 70ms/1000ms periods.
The sound worker's finite polling work runs cooperatively; actual MIDI/WAV
rendering runs on SDL's audio thread.

## Recovered class roles

| Class | Role |
|---|---|
| Zelda | MIDlet lifecycle and softkey commands |
| k | Main game canvas, menus, input, progression, save slots |
| j | World/map loading, collision, transitions and entity updates |
| h | Player, inventory, progression flags, original save encoder/decoder |
| l | Enemy and boss behavior/rendering |
| c | Collectible and equipment behavior |
| n | Map objects, triggers, scenery and scripted interactions |
| o | Sprite/resource catalogs |
| p | Projectiles and visual effects |
| f | Common drawable position and bounds |
| a / m | One-second / 70ms timer callbacks |
| b | Music selection, sound effects, volume and effect priority |
| d / e / g / i | Small state records, portals, effects and room history |

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
172-byte player payload produced by the translated original encoder. Integers
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
