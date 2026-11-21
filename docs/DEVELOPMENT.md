# Building and contributing

The game targets Apple Silicon Macs. To build from source, you need Rust, the
Xcode command-line tools, and CMake. The bootstrap installs CMake inside the
project and fetches the locked Rust dependencies:

```sh
python3 scripts/bootstrap.py
python3 scripts/package.py
```

Open the resulting `dist/Zelda Mobile.app` in Finder. Once built, playing needs
no Java, emulator, development tools, or network connection.

For an existing prepared checkout:

```sh
cargo run --release --locked --offline
cargo test --locked --offline
```

SDL3 is built from source and linked statically. The packaged application is
locally signed and uses macOS system frameworks. The native Rust sources and font
atlases are included, so a normal build does not require Java or the original JAR.

## Editing the game

Maintain the Rust implementation directly in `src/game/`. The module guide in
the [porting notes](../reference/PORTING.md) maps the original classes to named
Rust files and describes the host-service boundary. Start with `canvas/controls.rs`
for input, `world/collision.rs` for movement, `world/projectiles.rs` for combat,
and `player_save.rs` for save data. `canvas.rs` and `world.rs` group the respective
submodules.
There are no generated Rust files and no source translation command.

## Recovering reference material

The original JAR and recovered Java are retained for inspection and optional
behavior comparisons. With Java/Javac installed, run:

```sh
python3 scripts/recover.py
```

This recovers the Java decompilation, original assets and font atlases. It never
writes Rust sources. The Java is reference material; Cargo compiles the
independently maintained Rust implementation.

## Checking changes

```sh
python3 scripts/validate.py
```

This runs unit tests, resource and audio checks, loading and rendering of all map
variants, save round trips, and gameplay/menu replays. With Java available, it
also compares native behavior and save bytes against the unmodified original JAR.
The sprite oracle also compares all 256 IDs in four catalogs under two visual
variants: 2,048 cases covering sprite pixels, shared cache slots, reuse and aging.
Sprite checksums compare opaque RGB and all alpha values; translucent RGB is
omitted because Java's image conversion rounds those colors.
Reports and frames are written under `target/`, using isolated saves.

See the [validation report](../reference/VALIDATION.md) for measured results and
coverage limits. The complete campaign has not been manually played from
beginning to end. MIDI instruments can sound different from an original phone's
synthesizer.

## Original J2ME reference

The unmodified game JAR and FreeJ2ME-Plus 1.53 remain available as development
references. Java is required to run the original J2ME version. Its phone controls
are arrows, Return for fire, Q/W for softkeys, the number keys, E/R for `*` / `#`,
and +/- for scaling. Emulator saves remain separate from native saves and are
not automatically imported.

Component sources and licenses are listed in the [third-party notices](../THIRD_PARTY.md).
