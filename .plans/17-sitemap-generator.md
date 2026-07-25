# Plan: Dynamic sitemap.xml generator

## Goal

Replace the hand-maintained `sitemap.xml` with a generator script that pulls
published content from Supabase so the dynamic `/content/<slug>` routes are
discoverable by search engines without manual editing.

## Background

- `.plans/16-sitemap-xml.md` shipped a static sitemap with only the 5 static
  routes; the dynamic `/content/:slug` route was intentionally omitted because
  slugs live in Supabase.
- `src/routes/mod.rs` defines `Route::ContentDetail { slug }` at `/content/:slug`
  → a real, canonical, crawlable URL per post (loads content by slug).
- `src/features/content/content_grouped.rs` uses `/content?tag_id=…&content_id=…`
  for in-page filtering — these are SPA query params, not distinct resources.
- Supabase access is the standard PostgREST API used by `content_sdk`:
  - URL: `{SUPABASE_URL}/rest/v1/<table>?<params>`
  - Auth: `apikey` + `Authorization: Bearer <anon_key>` headers
  - Tables: `content` (has `slug`, `status`, `updated_at`, `created_at`)

## Decision: tags not added as sitemap URLs

Tags drive in-page filtering via query params (`/content?tag_id=…`). Adding
those to the sitemap would create near-duplicate URLs of `/content` (same
HTML, different filter state) — bad for SEO. Instead, every published post is
listed individually by slug, which is how crawlers reach tag-grouped content.

## Tasks

- [x] 0. Create `scripts/update-sitemap.mjs`
      - Node ESM, no deps (uses built-in `fetch` — Node 18+)
      - Reads `SUPABASE_URL` / `SUPABASE_ANON_KEY` from `.env`
      - `GET content?select=slug,updated_at,created_at&status=eq.published`
      - Static routes hardcoded (mirrors `src/routes/mod.rs`)
      - Dynamic `/content/<slug>` per row, `lastmod = updated_at || created_at`
      - Dedupes by slug, XML-escapes loc, writes valid sitemap
- [x] 1. Run generator: 5 static + 39 content urls (44 total), `xmllint` ✅
- [x] 2. Idempotent re-run produces identical output ✅
- [x] 3. Update `dist/build.md`:
      - Build step 3 now runs `node scripts/update-sitemap.mjs` before `cp`
      - sitemap section documents the generator + why tag URLs are omitted

## Notes

- Canonical origin `https://www.lilidm.com/` is the default; override with
  `SITEMAP_BASE_URL` env var (e.g. for staging).
- Static `lastmod` uses today's date — fine since they are SPA shell routes.
- `priority`/`changefreq` are intentionally kept as constants; they are
  advisory hints and rarely move.
- `npm`/package.json not added — the script is plain Node with no deps, run
  directly via `node scripts/update-sitemap.mjs`.
- Did NOT touch `vercel.json` — the `/sitemap.xml` route already exists.
