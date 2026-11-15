#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
. "$root/scripts/macos-signing.sh"
: "${UTTER_NOTARY_PROFILE:?Set UTTER_NOTARY_PROFILE to the matching notarization profile}"
version=$(utter_version "$root")
arch=$(uname -m)
dist="$root/dist/service"
artifact="$dist/HEX-Service-$version-$arch.zip"
submission="$dist/HEX-Service-$version-$arch.notarization.zip"

bundle=$("$root/scripts/build-service-app.sh")
rm -rf "$dist"
mkdir -p "$dist"
/usr/bin/ditto -c -k --sequesterRsrc --keepParent "$bundle" "$submission"
utter_notarize "$submission"
xcrun stapler staple "$bundle"
xcrun stapler validate "$bundle"
rm -f "$submission"
/usr/bin/ditto -c -k --sequesterRsrc --keepParent "$bundle" "$artifact"
shasum -a 256 "$artifact" > "$artifact.sha256"

echo "$artifact"
