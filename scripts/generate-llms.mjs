#!/usr/bin/env node
// Generates llms.txt (https://llmstxt.org) — the file AI agents read to
// discover what content the site offers. Derived from the same snapshot the
// app embeds, so it always matches the published set.
// Run AFTER `make snapshot` (needs assets/content_snapshot.json).

import { readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const PROJECT_ROOT = path.resolve(path.dirname(__filename), "..");
const SNAPSHOT_PATH = path.join(PROJECT_ROOT, "assets", "content_snapshot.json");
const OUT_PATH = path.join(PROJECT_ROOT, "llms.txt");
const BASE_URL = (process.env.SITEMAP_BASE_URL ?? "https://www.lilidm.com").replace(/\/$/, "");
const EXCERPT_LENGTH = 120;

function excerpt(body) {
  const text = String(body ?? "")
    .replace(/^---[\s\S]*?---/, "") // strip frontmatter
    .replace(/<[^>]*>/g, " ")
    .replace(/!\[.*?\]\(.*?\)/g, " ")
    .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/[#*_>`~]/g, " ")
    .replace(/\s+/g, " ")
    .trim();
  return text.length <= EXCERPT_LENGTH ? text : `${text.slice(0, EXCERPT_LENGTH - 1).trimEnd()}…`;
}

const { content = [], tags = [] } = JSON.parse(await readFile(SNAPSHOT_PATH, "utf8"));
const PUBLISHED = "published";
const tagName = (id) => tags.find((t) => t.id === id)?.name ?? "";

const published = content
  .filter((c) => c.status === PUBLISHED && c.slug)
  .sort((a, b) => String(b.created_at ?? "").localeCompare(String(a.created_at ?? "")));

const lines = [
  "# Moo's Profile",
  "",
  `> Technical articles on Rust, embedded systems (ESP32), and databases by Moo. ${published.length} published articles, in English.`,
  "",
  "## Articles",
  "",
];

for (const c of published) {
  const date = c.created_at ? String(c.created_at).slice(0, 10) : "";
  lines.push(`- [${c.title}](${BASE_URL}/content/${encodeURIComponent(c.slug)}): ${excerpt(c.body)}${date ? ` (${date})` : ""}`);
}

lines.push("", "## Topics", "");
for (const t of tags) {
  if (t.id === null || t.id === undefined) continue;
  lines.push(`- ${t.name}`);
}

await writeFile(OUT_PATH, lines.join("\n") + "\n", "utf8");
console.log(`Wrote ${OUT_PATH} (${published.length} articles)`);
