#!/bin/sh
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
export UTTER_TEAM_ID=TESTTEAM00
temporary=$(mktemp -d "${TMPDIR:-/tmp}/utter-app-identity.XXXXXX")
trap 'rm -rf "$temporary"' EXIT HUP INT TERM
bundle="$temporary/Utter.app"
mkdir -p "$bundle/Contents"
plist="$bundle/Contents/Info.plist"
cp "$root/app/Info.plist" "$plist"

[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist")" = app.utter ]
[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$plist")" = utter ]
[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleDisplayName' "$plist")" = Utter ]
[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleName' "$plist")" = Utter ]

expect_rejection() {
  if output=$(sh "$root/scripts/validate-app.sh" "$bundle" Utter.app 2.1.0 20100 2>&1); then
    echo "Unexpectedly accepted an incompatible app." >&2
    exit 1
  fi
  printf '%s\n' "$output" | grep -Fq "$1"
}

/usr/libexec/PlistBuddy -c 'Set :CFBundleIdentifier app.utter.legacy' "$plist"
expect_rejection 'wrong bundle identifier'
/usr/libexec/PlistBuddy -c 'Set :CFBundleIdentifier app.utter.agent' "$plist"
expect_rejection 'wrong bundle identifier'
/usr/libexec/PlistBuddy -c 'Set :CFBundleIdentifier ly.anoma.Utter' "$plist"
expect_rejection 'wrong bundle identifier'
/usr/libexec/PlistBuddy -c 'Set :CFBundleIdentifier app.utter.v1' "$plist"
expect_rejection 'wrong bundle identifier'
/usr/libexec/PlistBuddy -c 'Set :CFBundleIdentifier app.utter' "$plist"
/usr/libexec/PlistBuddy -c 'Set :CFBundleExecutable utter-watch' "$plist"
expect_rejection 'wrong executable name'
/usr/libexec/PlistBuddy -c 'Set :CFBundleExecutable utter' "$plist"
/usr/libexec/PlistBuddy -c 'Set :CFBundleShortVersionString 2.1.0' "$plist"
/usr/libexec/PlistBuddy -c 'Set :CFBundleVersion 20100' "$plist"
mkdir -p "$bundle/Contents/MacOS"
printf '#!/bin/sh\n' >"$bundle/Contents/MacOS/utter"
chmod +x "$bundle/Contents/MacOS/utter"
expect_rejection 'scrawl-helper is missing'

for script in build-app.sh build-service-app.sh release-app.sh; do
  if output=$(env -u UTTER_TEAM_ID sh "$root/scripts/$script" 2>&1); then
    echo "Unexpectedly allowed $script without an explicit signing team." >&2
    exit 1
  fi
  printf '%s\n' "$output" | grep -Fq 'Set UTTER_TEAM_ID to the Apple Developer signing team'
done

for script in build-app.sh build-service-app.sh; do
  for identity in 'Developer ID Application: Fixture (WRONGTEAM0)' 'Apple Development: Fixture (TESTTEAM00)'; do
    if output=$(UTTER_CODESIGN_IDENTITY="$identity" sh "$root/scripts/$script" 2>&1); then
      echo "Unexpectedly allowed an incompatible signing identity in $script." >&2
      exit 1
    fi
    printf '%s\n' "$output" | grep -Fq 'Signing identity is not a Developer ID Application identity for team TESTTEAM00'
  done
done

if output=$(env -u UTTER_NOTARY_PROFILE sh "$root/scripts/prepare-service-app.sh" 2>&1); then
  echo "Unexpectedly allowed helper notarization without an explicit profile." >&2
  exit 1
fi
printf '%s\n' "$output" | grep -Fq 'Set UTTER_NOTARY_PROFILE to the matching notarization profile'

echo "Rust app identity guards passed."
