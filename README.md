# Zelda Mobile — native Rust version

Open **Zelda Mobile** in Applications, or open a local build at
`dist/Zelda Mobile.app`.
The native Apple Silicon app includes the complete supplied beta's game logic,
maps, graphics and music. Playing requires no Java, emulator, Cargo, or network.

## Controls

| Action | MacBook keyboard | Xbox controller |
|---|---|---|
| Move / navigate | Arrow keys | D-pad or left stick |
| Primary action / confirm | Z | A |
| Secondary action / back | X | B |
| Inventory / game menu | Enter | Menu / Start |
| Left command shown at bottom | Q | LB |
| Right command shown at bottom | W | RB |
| Configure native controls | F2 | View / Back |
| Return / back | Escape | B, or the displayed Return command |
| Fullscreen | F11 | — |
| Screenshot | F12 | — |

Use **Z / A** on the title screen, then **Z / A** on a save slot to start.
The bottom command labels remain usable with Q/W or LB/RB in every game screen.
The original beta includes its own configuration screens; F2/View opens the
native keyboard/controller configuration.

In native controls, use Up/Down or the D-pad to select an action. Tab/Y switches
between keyboard and controller bindings. Z/A starts capture; press the desired
key/button. Escape cancels capture. R restores defaults. Remappings save immediately.
The controller can be connected before or after launch. Keyboard and controller
work together; the stick has a dead zone to reject drift.

The original 240×320 presentation is preserved with sharp scaling. The game scene
uses its original 256×176 maps and HUD; unused space below the scene is intentional.
Focus loss pauses the game/audio and releases held controls.

## Saves

Native saves and settings live in:

```
~/Library/Application Support/Zelda Mobile/
```

There are three original character slots. Use the original inventory **SAVE** or
**Save & exit** commands; closing the native window also saves an active character.
`controls.json` stores native bindings. F12 writes `Screenshot.png` in this folder.
Emulator saves remain separate and are not automatically imported.

## Original J2ME version

The unmodified JAR and FreeJ2ME-Plus 1.53 remain available as a development
reference. Keep `emulator/` to preserve its settings and saves.
Its phone controls are arrows, Return (fire), Q/W (softkeys), number keys,
E/R (`*` / `#`), and +/- for scaling. Java is required to run the original J2ME version.

## Build

Rust, the Xcode command-line tools, and CMake are needed to build. The bootstrap
installs CMake inside this project and fetches the locked Rust dependencies:

```sh
python3 scripts/bootstrap.py
python3 scripts/package.py
```

For an existing prepared checkout:

```sh
cargo run --release --locked --offline
cargo test --locked --offline
```

SDL3 is built from source and linked statically. The resulting `.app` is locally
signed and uses only macOS system frameworks. The supplied generated Rust and
font atlases allow normal Rust builds without Java.

To reproduce the decompilation, resource extraction, font capture, and Rust
translation, run `python3 scripts/recover.py` with Java/Javac installed. See
[the porting notes](reference/PORTING.md) for the architecture and recovered formats.

## Validation

`python3 scripts/validate.py` runs the resource/audio audit, all-map game-loader
audit, original save round trips, opening replay, and unit tests. With Java it also
compares native behavior and save bytes against the unmodified original JAR.
Reports and comparison frames are written under `target/` using isolated saves.

All 505 maps, 240 original PNGs and 11 MIDI files have been checked. The opening
gameplay frame and seven behavior/save snapshots have been compared to the
original. See [validation details](reference/VALIDATION.md) for coverage and limits.

Original beta behavior, test rooms, spelling and credits are retained. The entire
campaign has not been manually played from beginning to end. MIDI instruments can
sound different from a particular phone's synthesizer.

See [third-party notices](THIRD_PARTY.md) for component sources and licenses.
