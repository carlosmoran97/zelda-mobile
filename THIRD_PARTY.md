# Original game and native dependencies

The original JAR identifies Zelda Mobile version 4.12 (beta 4), by **Gorthwogh**.
Its original artwork, maps, music, credits, and game logic are retained. The native
application is a local restoration of that supplied game; this project does not
assign a new license to the original game or its resources.

- [CFR 0.152](https://www.benf.org/other/cfr/) is a development-only decompiler.
- [FreeJ2ME-Plus 1.53](https://github.com/TASEmulators/freej2me-plus/releases/tag/1.53)
  is the unmodified GPL-3.0 development reference. It is **not bundled in the
  native application**. Its emulator files remain available separately.
- [SDL3](https://github.com/libsdl-org/SDL) supplies native macOS rendering, input,
  controller support and audio output. It is linked statically under its zlib license.
- [RustySynth](https://github.com/sinshu/rustysynth) supplies MIDI synthesis (MIT).
- [GeneralUser GS 2.0.3](https://github.com/mrbumpy409/GeneralUser-GS) supplies MIDI
  instruments. Its full license is in `assets/audio/LICENSE-GeneralUser-GS.txt`.
- The original platform's font glyphs and metrics were captured during development
  from the installed Mac/Java font services; runtime playback uses those PNG atlases.
  The native control settings screen uses the Mac's installed Arial fonts.

The packaged app includes dependency license files under `Contents/Resources/Licenses`.

The browser build shares the maintained Rust game and original resource atlases.
It uses [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen) (MIT/Apache-2.0)
for browser bindings, the same RustySynth synthesizer in a Web Worker, and Web
Audio for output. SDL3 and the macOS system fonts are not part of the web build.
The GeneralUser GS license is included beside the downloadable soundfont and
linked from the browser credits page.
