#!/bin/sh

# Sourced by the app and helper packagers. Print only the selected identity so
# callers can capture it without mixing diagnostics into codesign arguments.
utter_codesign_identity() {
  team_id=$1
  identity=${UTTER_CODESIGN_IDENTITY:-}
  if [ -z "$identity" ]; then
    identity=$(security find-identity -v -p codesigning | sed -n "s/.*\"\(Developer ID Application:.*($team_id)\)\"/\1/p" | head -1)
  fi
  if [ -z "$identity" ]; then
    echo "No Developer ID signing identity found for team $team_id. Set UTTER_CODESIGN_IDENTITY." >&2
    return 1
  fi
  case "$identity" in
    "Developer ID Application:"*"($team_id)") ;;
    *)
      echo "Signing identity is not a Developer ID Application identity for team $team_id." >&2
      return 1
      ;;
  esac
  printf '%s\n' "$identity"
}

# Submit one artifact and wait. A local notarytool keychain profile wins;
# otherwise CI authenticates with an App Store Connect API key file.
utter_notarize() {
  if [ -n "${UTTER_NOTARY_PROFILE:-}" ]; then
    xcrun notarytool submit "$1" --keychain-profile "$UTTER_NOTARY_PROFILE" --wait
  else
    xcrun notarytool submit "$1" \
      --key "${UTTER_NOTARY_KEY:?Set UTTER_NOTARY_PROFILE, or UTTER_NOTARY_KEY with UTTER_NOTARY_KEY_ID and UTTER_NOTARY_ISSUER}" \
      --key-id "${UTTER_NOTARY_KEY_ID:?Set UTTER_NOTARY_KEY_ID to the App Store Connect API key ID}" \
      --issuer "${UTTER_NOTARY_ISSUER:?Set UTTER_NOTARY_ISSUER to the App Store Connect issuer ID}" \
      --wait
  fi
}

utter_version() {
  printf '%s\n' "${UTTER_VERSION:-$(cargo metadata --no-deps --format-version 1 --manifest-path "$1/Cargo.toml" | jq -r '.packages[] | select(.name == "utter") | .version')}"
}

# CFBundleVersion: the run number of a dev build (0.0.0-dev-N), else from
# X.Y.Z (a prerelease shares its release's number). Utter's updater compares
# neither; it follows the channel's current release.
utter_build_number() {
  if [ -n "${UTTER_BUILD_NUMBER:-}" ]; then
    printf '%s\n' "$UTTER_BUILD_NUMBER"
    return
  fi
  case "$1" in
    0.0.0-dev-[0-9]*) printf '%s\n' "${1#0.0.0-dev-}" ;;
    *) printf '%s\n' "${1%%-*}" | awk -F. '{ print ($1 * 10000) + ($2 * 100) + $3 }' ;;
  esac
}
