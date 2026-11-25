import { releaseDownload, supportedPlatforms } from "../lib/releases.js";

export default async function handler(req, res) {
  if (req.method !== "GET" && req.method !== "HEAD") {
    res.setHeader("Allow", "GET, HEAD");
    return res.status(405).end();
  }
  const platform = req.query.platform;
  if (!supportedPlatforms.includes(platform)) {
    return res
      .status(400)
      .json({ error: "Choose a supported Mac or Windows download." });
  }
  try {
    const headers = {
      Accept: "application/vnd.github+json",
      "User-Agent": "Utter-downloads",
    };
    if (process.env.GITHUB_RELEASE_TOKEN)
      headers.Authorization = `Bearer ${process.env.GITHUB_RELEASE_TOKEN}`;
    const response = await fetch(
      "https://api.github.com/repos/Sudeepnt/Hex/releases/latest",
      {
        headers,
        signal: AbortSignal.timeout(8000),
      },
    );
    if (!response.ok) throw new Error("Release unavailable");
    const url = releaseDownload(await response.json(), platform);
    if (!url) throw new Error("Installer unavailable");
    res.setHeader("Cache-Control", "public, max-age=0, s-maxage=60");
    return res.redirect(302, url);
  } catch {
    res.setHeader("Cache-Control", "no-store");
    res.setHeader("Content-Type", "text/html; charset=utf-8");
    return res
      .status(503)
      .send(
        '<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>Utter download</title><link rel="stylesheet" href="/style.css"><body><main class="unavailable"><h1>This download is being prepared.</h1><p>A published installer is not available right now. Please try again later.</p><a class="button primary" href="/">Back to Utter</a></main></body></html>',
      );
  }
}
