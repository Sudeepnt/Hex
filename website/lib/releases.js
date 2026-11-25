const platforms = {
  "macos-arm64": /^utter-\d+\.\d+\.\d+-macos-arm64\.dmg$/,
  "windows-x64": /^utter-setup-\d+\.\d+\.\d+-windows-x64\.exe$/,
};
export function releaseDownload(release, platform) {
  if (!Object.hasOwn(platforms, platform)) return null;
  if (release.draft || release.prerelease) return null;
  const asset = release.assets?.find(
    (item) =>
      platforms[platform].test(item.name) &&
      item.state === "uploaded" &&
      item.size > 0,
  );
  if (!asset) return null;
  const url = new URL(asset.browser_download_url);
  if (
    url.origin !== "https://github.com" ||
    !url.pathname.startsWith("/Sudeepnt/Hex/releases/download/")
  )
    return null;
  return url.href;
}
export const supportedPlatforms = Object.keys(platforms);
