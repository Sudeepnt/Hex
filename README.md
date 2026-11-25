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
- Icons: replace `assets/icons/utter-logo.png`, then run `swift scripts/render-icons.swift`.
  Copy the same artwork to `website/public/assets/utter-logo.png` for the website.
- Fonts: `crates/utter-theme/assets/fonts` (Figtree and JetBrains Mono, SIL OFL).

## License

MIT. See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

## Downloads and website

The Vercel website is in `website/`. Set the Vercel project's Root Directory to
`website` when importing this Git repository. The checked-in `vercel.json` supplies
the build command and output directory. Node 22 or later is sufficient.
Run `npm run build` and `npm test` there to verify the assets and download handler.
The website uses GitHub Releases to host installers, keeping binaries out of Git.

The Mac and Windows buttons request the latest published, non-prerelease GitHub
Release from `Sudeepnt/Hex` and redirect to its matching installer asset. Until a
matching release exists, the website shows a download preparation page. A private
repository needs a server-only `GITHUB_RELEASE_TOKEN` with read access to releases.
The installer download URLs must be publicly accessible for visitors to download
without GitHub authentication. A public release repository is recommended for
public distribution. Never add a token to browser code or Git.

### Mac distribution

`./scripts/package-macos.sh` builds an Apple silicon disk image, signs the app,
notarizes and staples both app and disk image, and runs Gatekeeper validation.
The image includes an Applications shortcut for drag-to-install setup. Its name is
`utter-<version>-macos-arm64.dmg`, accompanied by a SHA-256 checksum.
Use an official Rust toolchain and a full Xcode installation for public builds.
The release targets macOS 15 or later. Test on the oldest supported macOS version
before publication. Xcode is needed for Apple Intelligence integration.

Configure `UTTER_TEAM_ID` and a Developer ID Application certificate in Keychain,
plus either `UTTER_NOTARY_PROFILE` or `UTTER_NOTARY_KEY`, `UTTER_NOTARY_KEY_ID`, and
`UTTER_NOTARY_ISSUER`. `./scripts/package-macos.sh --local` produces a separately
named, ad-hoc-signed preview for local testing. It cannot pass the public release
validation. Both builds disable automatic updates while a release service is
unconfigured.

The manually triggered `Build downloadable installers` GitHub workflow produces
Mac and Windows artifacts and runs the website checks. Set these repository
secrets for its Mac job before running it.

- `MAC_CERTIFICATE_P12_BASE64` and `MAC_CERTIFICATE_PASSWORD`
- `MAC_TEAM_ID`
- `MAC_NOTARY_KEY`, `MAC_NOTARY_KEY_ID`, and `MAC_NOTARY_ISSUER`

The workflow exports artifacts for review. After clean-machine validation, create
a GitHub Release with the matching version tag and upload the `.dmg`, Windows
`utter-setup-<version>-windows-x64.exe`, and checksums. Publish the release to make
the website buttons start downloads. Keep unsigned previews out of public releases.

### Windows distribution

The workflow builds with the Vulkan SDK and wraps the GUI and console executables
using the existing Inno Setup installer. It exercises install, reinstall and
uninstall on a disposable Windows runner. The Windows artifact currently has no
Authenticode signature. Configure Windows code signing before public distribution
if you want a verified publisher. Run a manual Windows first-launch and dictation
check before publication.

### Pricing preview

`website/public/config.js` contains the display prices. Monthly is US$0.50 per month. Lifetime is US$50. The plan buttons open a purchase preview. Checkout, billing, receipts,
subscription handling, and app license enforcement still need the payment
integration. Downloads currently require no purchase. The existing MIT source
license remains applicable.

### Release verification remaining

A public release requires Apple signing and notarization credentials, a successful
Windows workflow, and published installer assets. Verify model download, microphone
permission, Accessibility permission, shortcut capture, dictation into another app,
and relaunch on a clean Mac, including macOS 15. No permissions are granted by a
build or smoke test. The website can be deployed independently while downloads are
being prepared.
