#!/usr/bin/env bash
# Public builds require Developer ID signing and notarization. --local creates
# an explicitly unsigned preview for this machine and manual testing only.
set -euo pipefail
root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
cd "$root"
export UTTER_VERSION=${UTTER_VERSION:-$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -1)}
export UTTER_LOCAL_BUILD=1
export CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_STRIP=false
export TRANSCRIBE_CMAKE_ARGS="${TRANSCRIBE_CMAKE_ARGS:-} -DGGML_NATIVE=OFF"
if [ "${1:-}" = --local ]; then
  export UTTER_RELEASE_UNSIGNED=1
  suffix=-local-unsigned
else
  : "${UTTER_TEAM_ID:?Public releases require an Apple Developer team}"
  if [ -z "${UTTER_NOTARY_PROFILE:-}${UTTER_NOTARY_KEY:-}" ]; then
    echo 'Public releases require notarization credentials.' >&2
    exit 1
  fi
  suffix=
fi
./scripts/build-app.sh
bundle="$root/target/app/Utter.app"
mkdir -p dist
image="$root/dist/utter-$UTTER_VERSION-macos-arm64$suffix.dmg"
stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
cp -R "$bundle" "$stage/Utter.app"
ln -s /Applications "$stage/Applications"
if [ -z "$suffix" ]; then
  . "$root/scripts/macos-signing.sh"
  build=$(utter_build_number "$UTTER_VERSION")
  ./scripts/validate-app.sh "$bundle" Utter.app "$UTTER_VERSION" "$build"
  ditto -c -k --keepParent "$bundle" "$stage/notarize.zip"
  utter_notarize "$stage/notarize.zip"
  rm "$stage/notarize.zip"
  xcrun stapler staple "$bundle"
  rm -rf "$stage/Utter.app"
  cp -R "$bundle" "$stage/Utter.app"
fi
hdiutil create -volname Utter -srcfolder "$stage" -ov -format UDZO "$image"
if [ -z "$suffix" ]; then
  sign_id=$(utter_codesign_identity "$UTTER_TEAM_ID")
  codesign --force --timestamp --sign "$sign_id" "$image"
  utter_notarize "$image"
  xcrun stapler staple "$image"
  xcrun stapler validate "$image"
  spctl --assess --type execute --verbose=2 "$bundle"
fi
(cd dist && shasum -a 256 "$(basename "$image")" > "$(basename "$image").sha256")
printf '%s\n' "$image"
