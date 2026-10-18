#!/usr/bin/env node
// Generates the ARD manifest (https://agenticresourcediscovery.org) for AI
// agent discovery: dist/public/.well-known/ard.json plus the predecessor
// path ai-catalog.json (served identically). Derived from the same snapshot
// the app embeds, so it always matches the published set.
// Run AFTER `make snapshot` (needs assets/content_snapshot.json).

import { mkdir, readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const PROJECT_ROOT = path.resolve(path.dirname(__filename), "..");
const SNAPSHOT_PATH = path.join(PROJECT_ROOT, "assets", "content_snapshot.json");
const WELL_KNOWN_DIR = path.join(PROJECT_ROOT, "dist", "public", ".well-known");
const BASE_URL = (process.env.SITEMAP_BASE_URL ?? "https://www.lilidm.com").replace(/\/$/, "");
const PUBLISHER = new URL(BASE_URL).hostname;
const EXCERPT_LENGTH = 160;

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

const { content = [] } = JSON.parse(await readFile(SNAPSHOT_PATH, "utf8"));
const PUBLISHED = "published";

const published = content
  .filter((c) => c.status === PUBLISHED && c.slug)
  .sort((a, b) => String(b.created_at ?? "").localeCompare(String(a.created_at ?? "")));

const entries = published.map((c) => ({
  identifier: `urn:air:${PUBLISHER}:content:${c.slug}`,
  displayName: c.title,
  type: 'text/markdown; profile="urn:air:agent-skills"',
  url: `${BASE_URL}/content/${encodeURIComponent(c.slug)}`,
  description: excerpt(c.body),
  representativeQueries: [
    `read the article "${c.title}"`,
    `what does Moo say about ${c.title}?`,
  ],
  updatedAt: c.updated_at ?? c.created_at,
  metadata: { kind: "article" },
}));

const manifest = {
  specVersion: "1.0",
  entries,
};

await mkdir(WELL_KNOWN_DIR, { recursive: true });
const json = JSON.stringify(manifest, null, 2);
await writeFile(path.join(WELL_KNOWN_DIR, "ard.json"), json, "utf8");
await writeFile(path.join(WELL_KNOWN_DIR, "ai-catalog.json"), json, "utf8");
console.log(`Wrote .well-known/ard.json + ai-catalog.json (${entries.length} entries)`);
