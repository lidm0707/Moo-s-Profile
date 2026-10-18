#!/usr/bin/env node
// Fetches ALL published content, tags, and content_tag rows from Supabase and
// writes assets/content_snapshot.json.
//
// The WASM app embeds that file at compile time (src/snapshot.rs), so the
// built site serves from CDN with zero runtime API calls. Data changes only
// appear after re-running this script and rebuilding.
//
// Reads SUPABASE_URL / SUPABASE_ANON_KEY from .env (same as update-sitemap).
// Run BEFORE `dx bundle` (the snapshot must exist when cargo compiles).

import { readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const PROJECT_ROOT = path.resolve(path.dirname(__filename), "..");
const ENV_PATH = path.join(PROJECT_ROOT, ".env");
const OUT_PATH = path.join(PROJECT_ROOT, "assets", "content_snapshot.json");

const REST_PATH = "rest/v1";
const PUBLISHED_STATUS = "published";
const PAGE_SIZE = 1000; // PostgREST hard row limit per request

function parseEnv(envText) {
  const env = {};
  for (const rawLine of envText.split("\n")) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) continue;
    const eq = line.indexOf("=");
    if (eq === -1) continue;
    const key = line.slice(0, eq).trim();
    let value = line.slice(eq + 1).trim();
    if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
      value = value.slice(1, -1);
    }
    env[key] = value;
  }
  return env;
}

async function loadSupabaseCreds() {
  try {
    const env = parseEnv(await readFile(ENV_PATH, "utf8"));
    return {
      baseUrl: (process.env.SUPABASE_URL ?? env.SUPABASE_URL ?? "").replace(/\/$/, ""),
      anonKey: process.env.SUPABASE_ANON_KEY ?? env.SUPABASE_ANON_KEY,
    };
  } catch {
    return { baseUrl: process.env.SUPABASE_URL ?? "", anonKey: process.env.SUPABASE_ANON_KEY };
  }
}

async function fetchAllRows(baseUrl, anonKey, table, query) {
  const rows = [];
  let offset = 0;
  for (;;) {
    const qs = `${query}&limit=${PAGE_SIZE}&offset=${offset}`;
    const res = await fetch(`${baseUrl}/${REST_PATH}/${table}?${qs}`, {
      headers: { apikey: anonKey, Authorization: `Bearer ${anonKey}`, Accept: "application/json" },
    });
    if (!res.ok) {
      throw new Error(`${table}: HTTP ${res.status} ${await res.text()}`);
    }
    const page = await res.json();
    rows.push(...page);
    if (page.length < PAGE_SIZE) return rows;
    offset += PAGE_SIZE;
  }
}

async function main() {
  const { baseUrl, anonKey } = await loadSupabaseCreds();
  if (!baseUrl || !anonKey) {
    console.error("Missing SUPABASE_URL / SUPABASE_ANON_KEY (env or .env)");
    process.exit(1);
  }

  const [content, tags, contentTags] = await Promise.all([
    fetchAllRows(baseUrl, anonKey, "content", `select=*&status=eq.${PUBLISHED_STATUS}&order=created_at.desc`),
    fetchAllRows(baseUrl, anonKey, "tags", "select=*"),
    fetchAllRows(baseUrl, anonKey, "content_tags", "select=*"),
  ]);

  const snapshot = { content, tags, content_tags: contentTags };
  await writeFile(OUT_PATH, JSON.stringify(snapshot));
  console.log(
    `Snapshot written: ${content.length} content, ${tags.length} tags, ${contentTags.length} links -> assets/content_snapshot.json`,
  );
}

main().catch((err) => {
  console.error(err.message);
  process.exit(1);
});
