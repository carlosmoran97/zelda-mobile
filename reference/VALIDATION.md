# Validation coverage — 2026-11-21

The repeatable checks are implemented in `scripts/validate.py`. They use isolated
save directories and the unmodified supplied JAR as a development-only oracle.

| Check | Result |
|---|---|
| Original archive resources compared byte for byte | 757 / 757 identical |
| Strict XMOB parsing, including complete consumption | 505 / 505 pass |
| Maps loaded and painted through translated original game logic | 505 / 505 pass |
| Original PNG decoding | 240 / 240 pass |
| Original MIDI parsing and sample rendering | 11 / 11 pass |
| Opening gameplay frame compared with original JAR | 0 differing pixels / 49,920 scene pixels |
| Opening, down/right movement, sword pickup, heart pickup, sword action, inventory | 7 / 7 snapshots match the original |
| Original serialized player bytes in those snapshots | All 172 bytes match for each snapshot |
| Original encoder/decoder and native storage round trips | All three character slots pass |
| Input aggregation, opposite directions, disconnection, dead zone | Unit tests pass |
| Signed arrays, Java PRNG, collection, drawing transforms/clipping, damaged data | Unit tests pass |
| Native macOS SDL window and audio-device initialization | Pass |
| Connected hardware enumeration | Xbox Series X Controller detected |
| Installed app: keyboard navigation, movement, inventory and return | Verified in the native window |
| Native key remapping and restoring defaults | Verified; Z → Space → Z |
| Controller mappings and connected-device status in controls screen | Verified |
| Fullscreen after the macOS transition settles | Portrait presentation and letterboxing verified |
| Standalone app away from the source folder | Launches and starts gameplay using only bundled resources |
| Local application signature and linked dependencies | Valid signature; only macOS system libraries/frameworks |

Font capture retains reference glyphs, character advances, line heights and
baselines for the original platform's 13/15/17-pixel fonts. Native rendering uses
these static atlases without Java.

The completed application was moved to `/Applications/Zelda Mobile.app`.
`Play Zelda.command` recognizes that installed location as well as local builds.

The reference game is run headlessly for deterministic graphics/state captures.
Its emulator audio services are not used as an audio oracle. Native MIDI playback
uses the original files with RustySynth and the bundled GeneralUser GS instruments.

## Limits

The complete campaign has not been manually played from its beginning to its end.
Loading and painting every room does not prove every possible puzzle, combat,
progression, or save-state combination. The original beta's test maps, unfinished
content and quirks are retained. No missing original story content is invented.

The connected Xbox controller is detected by the native SDL gamepad subsystem.
Automated checks cover logical mappings and input lifetime; physical button presses
and a full controller playthrough require hands-on use of the controller.

MIDI timbre can differ from the synthesizer used by an original handset. The notes,
tempo, selected tracks, looping and packed-WAV effect offsets use the original data.
