#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
team_id=${UTTER_TEAM_ID:?Set UTTER_TEAM_ID to the Apple Developer signing team}
. "$root/scripts/macos-signing.sh"
identity=$(utter_codesign_identity "$team_id")
bundle="$root/target/HEX Service.app"
executable="$bundle/Contents/MacOS/utter-service"
version=$(utter_version "$root")
build_number=$(utter_build_number "$version")

cargo build --release --manifest-path "$root/Cargo.toml"
rm -rf "$bundle"
mkdir -p "$bundle/Contents/MacOS"
cp "$root/target/release/utter" "$executable"
cp "$root/app/ServiceInfo.plist" "$bundle/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $version" "$bundle/Contents/Info.plist"
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $build_number" "$bundle/Contents/Info.plist"

codesign --force --timestamp --options runtime --sign "$identity" "$bundle"
codesign --verify --deep --strict --verbose=2 "$bundle"

echo "$bundle"
