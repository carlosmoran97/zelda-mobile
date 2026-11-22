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

## Browser build

Install Rust, Python 3.11+, and Node.js 22.12+ (including npm). The browser
interface is a client-side React app built with Vite, Tailwind CSS, and shadcn/ui.
The Rust game still runs locally through WebAssembly; there is no application server.
Build with:

```sh
python3 scripts/build_web.py --setup
python3 -m http.server 8080 --directory dist/web
```

Open `http://localhost:8080`. The setup option installs the
`wasm32-unknown-unknown` target and a matching, pinned `wasm-bindgen` CLI into
`target/web-tools`. Later builds can omit `--setup`; add `--offline` once Cargo
and npm dependencies have been fetched. The build installs the locked frontend
dependencies with `npm ci`, prepares `web/public/`, then runs Vite. No SDL, CMake,
Java, or macOS fonts are needed for the web build. Keep the HTTP server running;
opening the HTML file directly
does not support Wasm loading.

For React development with hot reload:

```sh
python3 scripts/build_web.py --prepare-only --setup
npm run dev --prefix web
```

Open the local URL printed by Vite. After changing Rust code, run the prepare
command again (without `--setup`) and refresh the browser. React components live
in `web/src/components/`, the game lifecycle in `web/src/game-session.js`, and
the React integration in `web/src/hooks/use-game.js`. `web/components.json`
configures shadcn with React Server Components disabled. To add a component,
run `npx shadcn@latest add <component>` from `web/`, following the
[shadcn Vite guide](https://ui.shadcn.com/docs/installation/vite).

The static output is `dist/web/`. Resource URLs are relative, including worker
and audio assets, so hosting beneath a repository path works. The large soundfont
is fetched only when sound is enabled. A dedicated worker runs the Rust synth at
the browser audio sample rate and transfers PCM blocks to an AudioWorklet.
No shared-memory or cross-origin isolation headers are required.

Run `npm test --prefix web` after building for deterministic tests of
the actual Wasm game, persistent records, input arbitration, touch helpers, and
audio queue, React mount/unmount cleanup, and pause/resume behavior.
`src/assets.rs` and `storage::RecordStore` form the host-service
boundary; native construction continues to use filesystem-backed services.

The **Browser game** Actions workflow checks pull requests and publishes `main`
to GitHub Pages. Pages must use **GitHub Actions** as its publishing source.
To inspect deployment, use `gh run list --workflow pages.yml` and
`gh run watch <run-id> --exit-status`. Browser smoke checks should cover mobile
Safari and Chrome, multi-touch movement/actions, save/reload, audio, and app
switching on the deployed HTTPS address.

## Editing the game

Maintain the Rust implementation directly in `src/game/`. The module guide in
the [porting notes](../reference/PORTING.md) maps the original classes to named
Rust files and describes the host-service boundary. Start with `canvas/controls.rs`
for input, `world/collision.rs` for movement, `world/projectiles.rs` for combat,
and `player_save.rs` for save data. `canvas.rs` and `world.rs` group the respective
submodules.
There are no generated Rust files and no source translation command.

## Recovering reference material

The original JAR, emulator, recovered Java, comparison harnesses and decompiler
tooling live in [`java/`](../java/README.md) for inspection and optional behavior
comparisons. With Java/Javac installed, run:

```sh
python3 java/recover.py
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
Reports and frames are written under `target/`, using isolated saves. Java
classes and emulator working files are written under `java/target/`.

See the [validation report](../reference/VALIDATION.md) for measured results and
coverage limits. The complete campaign has not been manually played from
beginning to end. MIDI instruments can sound different from an original phone's
synthesizer.

## Original J2ME reference

The unmodified game JAR and FreeJ2ME-Plus 1.53 remain available under `java/` as
development references. Java is required to run the original J2ME version. Its
phone controls are arrows, Return for fire, Q/W for softkeys, the number keys, E/R for `*` / `#`,
and +/- for scaling. Emulator saves remain separate from native saves and are
not automatically imported.

Component sources and licenses are listed in the [third-party notices](../THIRD_PARTY.md).
