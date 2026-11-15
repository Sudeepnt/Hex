#!/bin/sh
# Builds `Utter Playground.app`: the Scrawl capture capsule in a window of its
# own (`utter capsule-playground`), driven by a fake host with live tuning.
# It touches no Utter data, settings, or captures, and records and sends
# nothing, so it runs beside the installed Utter. Ad-hoc signed, for this Mac.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
export MACOSX_DEPLOYMENT_TARGET=15.0
cargo build --locked --release --bin utter --manifest-path "$root/Cargo.toml"
target=${CARGO_TARGET_DIR:-$root/target}
bundle="${UTTER_PLAYGROUND_OUT:-$root/target/app}/Utter Playground.app"
rm -rf "$bundle"
mkdir -p "$bundle/Contents/MacOS" "$bundle/Contents/Resources"
cp "$target/release/utter" "$bundle/Contents/MacOS/utter"
cat > "$bundle/Contents/MacOS/playground" <<'SH'
#!/bin/sh
exec "$(dirname "$0")/utter" capsule-playground
SH
chmod +x "$bundle/Contents/MacOS/playground"
cat > "$bundle/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key><string>playground</string>
  <key>CFBundleIdentifier</key><string>app.utter.playground</string>
  <key>CFBundleName</key><string>Utter Playground</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST
codesign --force --sign - "$bundle/Contents/MacOS/utter"
codesign --force --sign - "$bundle"
echo "$bundle"
