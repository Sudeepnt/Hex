# Utter

Private voice dictation for Mac, Windows and Linux. Hold a key, speak, let go:
Utter writes what you said into whatever app you're in.

- **Local.** Speech is transcribed on your device. The first run downloads a
  speech model; after that nothing leaves your machine.
- **Fast.** Words appear as you speak, and paste the moment you let go.
- **Yours.** No account, no telemetry, no update server. Free and open source.

## Build from source

macOS (Apple silicon) needs Xcode and Rust:

```sh
CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_STRIP=false UTTER_TEST_BUILD=1 ./scripts/build-app.sh
open "target/app/Utter Test.app"
```

`cargo run -- app` starts the desktop app from a checkout on any OS.

## Command line

`utter transcribe meeting.m4a` prints a recording's transcript. It reads WAV,
AIFF, CAF, FLAC, MP3, Ogg, WebM and MP4/M4A/MOV. The same `utter` controls the
running app: `utter status`, `utter dictation`, `utter settings`.

## Make it yours

- Colours: edit the three values at the top of
  `crates/utter-theme/tools/brand.py` and run it.
- Icons: edit `scripts/render-icons.swift` and run `swift scripts/render-icons.swift`.
- Fonts: `crates/utter-theme/assets/fonts` (Figtree and JetBrains Mono, SIL OFL).

## License

MIT. See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
