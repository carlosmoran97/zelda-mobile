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
locally signed and uses macOS system frameworks. The generated Rust and font
atlases are included, so a normal build does not require Java.

## Reproducing the port

The original JAR, recovered Java, extracted resources and generated Rust are
retained for inspection. With Java/Javac installed, run:

```sh
python3 scripts/recover.py
```

This reproduces the decompilation, resource extraction, font capture and Rust
translation. See the [porting notes](../reference/PORTING.md) for the architecture,
class roles, map format and save encoding.

## Checking changes

```sh
python3 scripts/validate.py
```

This runs unit tests, resource and audio checks, loading and rendering of all map
variants, save round trips, and gameplay/menu replays. With Java available, it
also compares native behavior and save bytes against the unmodified original JAR.
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
