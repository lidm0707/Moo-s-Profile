#!/usr/bin/env node
// Pre-renders per-content meta pages: dist/public/content/<slug>/index.html
//
// Link unfurlers (Discord, LINE, X...) don't run JS/Wasm, so a shared SPA
// index.html can't produce per-route previews. Since the app is fully
// client-side, we instead write one small static HTML per published slug,
// based on the bundled index.html, with OG/Twitter tags replaced.
//
// Reads SUPABASE_URL / SUPABASE_ANON_KEY from .env (same as update-sitemap).
// Run AFTER `dx bundle` (needs the hashed asset links in dist/public/index.html).

import { readFile, writeFile, mkdir } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const PROJECT_ROOT = path.resolve(path.dirname(__filename), "..");
const ENV_PATH = path.join(PROJECT_ROOT, ".env");
const DIST_INDEX = path.join(PROJECT_ROOT, "dist", "public", "index.html");
const CONTENT_DIR = path.join(PROJECT_ROOT, "dist", "public", "content");

const REST_PATH = "rest/v1";
const PUBLISHED_STATUS = "published";
const BASE_URL = (process.env.SITEMAP_BASE_URL ?? "https://www.lilidm.com").replace(/\/$/, "");
const DEFAULT_OG_IMAGE = `${BASE_URL}/og-image.jpg`;
const EXCERPT_LENGTH = 160;

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

async function fetchPublishedContent({ baseUrl, anonKey }) {
  const qs = "select=slug,title,body&status=eq.published&order=created_at.desc";
  const res = await fetch(`${baseUrl}/${REST_PATH}/content?${qs}`, {
    headers: { apikey: anonKey, Authorization: `Bearer ${anonKey}`, Accept: "application/json" },
  });
  if (!res.ok) {
    throw new Error(`Supabase GET content failed: ${res.status} ${res.statusText}\n${await res.text()}`);
  }
  return res.json();
}

function bodyToExcerpt(body) {
  const text = String(body ?? "")
    .replace(/<[^>]*>/g, " ") // strip html tags
    .replace(/!\[.*?\]\(.*?\)/g, " ") // strip markdown images
    .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1") // markdown links -> text
    .replace(/[#*_>`~]/g, " ")
    .replace(/\s+/g, " ")
    .trim();
  if (text.length <= EXCERPT_LENGTH) return text;
  return `${text.slice(0, EXCERPT_LENGTH - 1).trimEnd()}…`;
}

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&apos;");
}

function metaBlock({ title, description, url, image }) {
  return [
    `<title>${escapeHtml(title)}</title>`,
    `<meta name="description" content="${escapeHtml(description)}">`,
    `<meta property="og:type" content="article">`,
    `<meta property="og:site_name" content="Moo's Profile">`,
    `<meta property="og:title" content="${escapeHtml(title)}">`,
    `<meta property="og:description" content="${escapeHtml(description)}">`,
    `<meta property="og:url" content="${escapeHtml(url)}">`,
    `<meta property="og:image" content="${escapeHtml(image)}">`,
    `<meta name="twitter:card" content="summary_large_image">`,
    `<meta name="twitter:title" content="${escapeHtml(title)}">`,
    `<meta name="twitter:description" content="${escapeHtml(description)}">`,
    `<meta name="twitter:image" content="${escapeHtml(image)}">`,
  ].join("\n        ");
}

function injectMeta(templateHtml, metaTags) {
  let html = templateHtml;
  // Replace the generic <title>…
  html = html.replace(/<title>[\s\S]*?<\/title>/, metaTags.split("\n")[0]);
  // Replace the generic description…
  html = html.replace(/<meta name="description"[^>]*>/, metaTags.split("\n")[1]);
  // Replace the whole generic OG/Twitter block (between the marker comment and the twitter:image tag)
  html = html.replace(
    /<!-- Link preview[\s\S]*?<meta name="twitter:image"[^>]*>/,
    metaTags.split("\n").slice(2).join("\n        "),
  );
  return html;
}

async function main() {
  const creds = await loadSupabaseCreds();
  if (!creds.baseUrl || !creds.anonKey) {
    throw new Error("SUPABASE_URL and SUPABASE_ANON_KEY must be set (env or .env)");
  }

  // Root index.html carries the generic OG/Twitter block to swap per slug.
  const rootIndex = await readFile(path.join(PROJECT_ROOT, "index.html"), "utf8");
  // The bundled dist index.html carries the hashed css/js asset links.
  const bundledIndex = await readFile(DIST_INDEX, "utf8");
  const hashedAssets = bundledIndex.match(/<link[^>]*assets\/(?:main|my_profile)[^>]*>|<script[^>]*assets\/my_profile[^>]*><\/script>/g) ?? [];
  if (hashedAssets.length === 0) {
    throw new Error("No hashed asset tags found in dist/public/index.html — run `dx bundle` first");
  }
  const template = rootIndex.replace(
    /<\/head>/,
    `${hashedAssets.join("\n        ")}\n    </head>`,
  );

  const rows = await fetchPublishedContent(creds);

  await mkdir(CONTENT_DIR, { recursive: true });
  let count = 0;
  for (const row of rows) {
    if (!row.slug) continue;
    const url = `${BASE_URL}/content/${encodeURIComponent(row.slug)}`;
    const meta = metaBlock({
      title: `${row.title} — Moo's Profile`,
      description: bodyToExcerpt(row.body),
      url,
      image: DEFAULT_OG_IMAGE,
    });
    const html = injectMeta(template, meta);
    const outDir = path.join(CONTENT_DIR, row.slug);
    await mkdir(outDir, { recursive: true });
    await writeFile(path.join(outDir, "index.html"), html, "utf8");
    count += 1;
  }
  console.log(`Wrote ${count} content meta pages to ${CONTENT_DIR}`);
}

main().catch((err) => {
  console.error("Failed to generate content meta pages:", err);
  process.exit(1);
});
