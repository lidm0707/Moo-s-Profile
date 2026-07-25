#!/usr/bin/env node
// Regenerates sitemap.xml at the project root.
//
// - Static routes (/, /interests, /work-history, /content, /chat) are kept in
//   sync with src/routes/mod.rs and use today's date as lastmod.
// - Dynamic routes (/content/<slug>) come from Supabase: every published
//   content row becomes one url entry, lastmod = updated_at || created_at.
//
// Reads SUPABASE_URL and SUPABASE_ANON_KEY from .env at the project root.
// Run: `node scripts/update-sitemap.mjs`
//
// Env: SITEMAP_BASE_URL (default https://www.lilidm.com) — canonical origin.

import { readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const PROJECT_ROOT = path.resolve(path.dirname(__filename), "..");
const ENV_PATH = path.join(PROJECT_ROOT, ".env");
const SITEMAP_PATH = path.join(PROJECT_ROOT, "sitemap.xml");

const BASE_URL = (process.env.SITEMAP_BASE_URL ?? "https://www.lilidm.com").replace(/\/$/, "");
const REST_PATH = "rest/v1";
const PUBLISHED_STATUS = "published";

// Static routes mirror src/routes/mod.rs. lastmod and changefreq/priority are
// baked in here so the static part of the sitemap stays deterministic.
const STATIC_ROUTES = [
  { path: "/", changefreq: "weekly", priority: "1.0" },
  { path: "/interests", changefreq: "monthly", priority: "0.6" },
  { path: "/work-history", changefreq: "monthly", priority: "0.6" },
  { path: "/content", changefreq: "weekly", priority: "0.8" },
  { path: "/chat", changefreq: "monthly", priority: "0.5" },
];

function todayIsoDate() {
  return new Date().toISOString().slice(0, 10);
}

function parseEnv(envText) {
  const env = {};
  for (const rawLine of envText.split("\n")) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) continue;
    const eq = line.indexOf("=");
    if (eq === -1) continue;
    const key = line.slice(0, eq).trim();
    let value = line.slice(eq + 1).trim();
    if (
      (value.startsWith('"') && value.endsWith('"')) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }
    env[key] = value;
  }
  return env;
}

async function loadEnv() {
  let envText = "";
  try {
    envText = await readFile(ENV_PATH, "utf8");
  } catch (err) {
    if (err.code === "ENOENT") {
      throw new Error(`Missing .env at ${ENV_PATH}. Add SUPABASE_URL and SUPABASE_ANON_KEY.`);
    }
    throw err;
  }
  return parseEnv(envText);
}

function buildSupabaseUrl(envSupabaseUrl, table, queryParams) {
  const base = envSupabaseUrl.replace(/\/$/, "");
  const qs = queryParams
    .map(([k, v]) => `${encodeURIComponent(k)}=${encodeURIComponent(v)}`)
    .join("&");
  return `${base}/${REST_PATH}/${table}?${qs}`;
}

async function supabaseGet({ baseUrl, anonKey, table, select, filters }) {
  const params = [["select", select]];
  for (const [k, v] of filters) {
    params.push([k, `eq.${v}`]);
  }
  const url = buildSupabaseUrl(baseUrl, table, params);

  const res = await fetch(url, {
    headers: {
      apikey: anonKey,
      Authorization: `Bearer ${anonKey}`,
      Accept: "application/json",
    },
  });

  if (!res.ok) {
    const body = await res.text();
    throw new Error(`Supabase GET ${table} failed: ${res.status} ${res.statusText}\n${body}`);
  }

  return res.json();
}

function escapeXml(value) {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&apos;");
}

function urlEntry({ loc, lastmod, changefreq, priority }) {
  return [
    "  <url>",
    `    <loc>${escapeXml(loc)}</loc>`,
    `    <lastmod>${lastmod}</lastmod>`,
    `    <changefreq>${changefreq}</changefreq>`,
    `    <priority>${priority}</priority>`,
    "  </url>",
  ].join("\n");
}

function buildSitemap(entries) {
  return [
    '<?xml version="1.0" encoding="UTF-8"?>',
    '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">',
    entries.join("\n"),
    "</urlset>",
    "",
  ].join("\n");
}

function lastmodFor(row) {
  const raw = row.updated_at || row.created_at;
  if (!raw) return todayIsoDate();
  // Supabase returns RFC3339; sitemap lastmod only needs the date.
  return String(raw).slice(0, 10);
}

async function main() {
  const env = await loadEnv();

  const supabaseUrl = process.env.SUPABASE_URL ?? env.SUPABASE_URL;
  const anonKey = process.env.SUPABASE_ANON_KEY ?? env.SUPABASE_ANON_KEY;

  if (!supabaseUrl || !anonKey) {
    throw new Error("SUPABASE_URL and SUPABASE_ANON_KEY must be set in .env");
  }

  console.log(`Fetching published content from ${supabaseUrl} ...`);

  // Pull just the fields the sitemap needs. status filter keeps drafts out.
  const contentRows = await supabaseGet({
    baseUrl: supabaseUrl,
    anonKey,
    table: "content",
    select: "slug,updated_at,created_at",
    filters: [["status", PUBLISHED_STATUS]],
  });

  const today = todayIsoDate();
  const entries = [];

  for (const route of STATIC_ROUTES) {
    entries.push(
      urlEntry({
        loc: `${BASE_URL}${route.path}`,
        lastmod: today,
        changefreq: route.changefreq,
        priority: route.priority,
      }),
    );
  }

  const seenSlugs = new Set();
  let dynamicCount = 0;
  for (const row of contentRows) {
    if (!row.slug || seenSlugs.has(row.slug)) continue;
    seenSlugs.add(row.slug);
    entries.push(
      urlEntry({
        loc: `${BASE_URL}/content/${row.slug}`,
        lastmod: lastmodFor(row),
        changefreq: "monthly",
        priority: "0.7",
      }),
    );
    dynamicCount += 1;
  }

  const xml = buildSitemap(entries);
  await writeFile(SITEMAP_PATH, xml, "utf8");

  console.log(
    `Wrote ${SITEMAP_PATH}: ${STATIC_ROUTES.length} static + ${dynamicCount} content urls ` +
      `(total ${entries.length}).`,
  );
}

main().catch((err) => {
  console.error("Failed to update sitemap:", err);
  process.exit(1);
});
