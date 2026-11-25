import test from "node:test";
import assert from "node:assert/strict";
import handler from "../api/download.js";
import { releaseDownload } from "../lib/releases.js";

function release(name = "utter-3.4.0-macos-arm64.dmg") {
  return {
    draft: false,
    prerelease: false,
    assets: [
      {
        name,
        state: "uploaded",
        size: 100,
        browser_download_url: `https://github.com/Sudeepnt/Hex/releases/download/v3.4.0/${name}`,
      },
    ],
  };
}
function response() {
  return {
    headers: {},
    setHeader(key, value) {
      this.headers[key] = value;
    },
    status(code) {
      this.code = code;
      return this;
    },
    json(body) {
      this.body = body;
      return this;
    },
    send(body) {
      this.body = body;
      return this;
    },
    end() {
      return this;
    },
    redirect(code, url) {
      this.code = code;
      this.url = url;
      return this;
    },
  };
}
test("only a published installer matching the platform can be downloaded", () => {
  assert.ok(releaseDownload(release(), "macos-arm64"));
  assert.equal(releaseDownload(release(), "windows-x64"), null);
  assert.ok(
    releaseDownload(
      release("utter-setup-3.4.0-windows-x64.exe"),
      "windows-x64",
    ),
  );
  for (const name of [
    "utter-3.4.0-macos-arm64-local-unsigned.dmg",
    "source.zip",
  ]) {
    assert.equal(releaseDownload(release(name), "macos-arm64"), null);
  }
  for (const field of ["draft", "prerelease"]) {
    assert.equal(
      releaseDownload({ ...release(), [field]: true }, "macos-arm64"),
      null,
    );
  }
  const untrusted = release();
  untrusted.assets[0].browser_download_url =
    "https://example.org/installer.dmg";
  assert.equal(releaseDownload(untrusted, "macos-arm64"), null);
});
test("download endpoint redirects to the installer and handles unavailable releases", async () => {
  const original = globalThis.fetch;
  try {
    globalThis.fetch = async () => ({ ok: true, json: async () => release() });
    const success = response();
    await handler(
      { method: "GET", query: { platform: "macos-arm64" } },
      success,
    );
    assert.equal(success.code, 302);
    assert.match(success.url, /releases\/download\/v3.4.0\/utter-3.4.0/);
    globalThis.fetch = async () => ({ ok: false });
    const unavailable = response();
    await handler(
      { method: "GET", query: { platform: "macos-arm64" } },
      unavailable,
    );
    assert.equal(unavailable.code, 503);
    assert.equal(unavailable.headers["Cache-Control"], "no-store");
    assert.match(unavailable.body, /being prepared/);
    const invalid = response();
    await handler({ method: "GET", query: { platform: "invalid" } }, invalid);
    assert.equal(invalid.code, 400);
    const post = response();
    await handler({ method: "POST", query: {} }, post);
    assert.equal(post.code, 405);
  } finally {
    globalThis.fetch = original;
  }
});
