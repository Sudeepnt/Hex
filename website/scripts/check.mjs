import { access, readFile } from "node:fs/promises";
const html = await readFile(
  new URL("../public/index.html", import.meta.url),
  "utf8",
);
for (const match of html.matchAll(
  /(?:src|href)="(\/[^"?#]+)(?:[?#][^"]*)?"/g,
)) {
  if (match[1].startsWith("/api/")) continue;
  await access(new URL(`../public${match[1]}`, import.meta.url));
}

console.log("Website assets verified.");
