# Java reference material

The original Java game and its development tools are kept here:

- `TheLegendOfZeldaMobile-beta4.jar`: the unmodified original game.
- `emulator/`: FreeJ2ME-Plus and its local configuration and saves.
- `reference/decompiled/`: recovered Java source for inspection.
- `reference/*.java`: behavior, sprite, frame and font capture harnesses.
- `reference/asset-manifest.json`: original resource sizes and checksums.
- `reference/translation.json`: historical Java-to-Rust translation coverage.
- `tools/`: the locally cached CFR decompiler.
- `target/`: ignored compiled harnesses and emulator working files.
- `freej2me_system/`: preserved local emulator settings.

From the repository root, recover the source, original assets and font atlases with:

```sh
python3 java/recover.py
```

Run native checks and Java comparisons with:

```sh
python3 scripts/validate.py
```

Both commands require Java and Javac for the Java steps. The recovery command
downloads CFR when it is not cached. Native and browser builds use the Rust
sources and bundled assets without these Java tools.

See the [development guide](../docs/DEVELOPMENT.md),
[porting notes](../reference/PORTING.md) and [third-party notices](../THIRD_PARTY.md).
