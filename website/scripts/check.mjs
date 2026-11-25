import { access, readFile } from "node:fs/promises";
import { plans } from "../public/config.js";
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
for (const plan of Object.values(plans)) {
  if (!Number.isFinite(Number(plan.amount)) || Number(plan.amount) <= 0)
    throw new Error("Invalid price");
}
console.log("Website assets and plan configuration verified.");
