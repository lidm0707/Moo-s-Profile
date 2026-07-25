# Plan: Add `sitemap.xml` for SEO

## Goal

Create a static `sitemap.xml` at the project root for search engine discovery,
wire it through `vercel.json` (so the SPA catch-all doesn't swallow it), and
document the manual-copy step in `dist/build.md` (same pattern as `ads.txt`).

## Background

- Routes from `src/routes/mod.rs`:
  - `/` (Home)
  - `/interests`
  - `/work-history`
  - `/content`
  - `/content/:slug` (dynamic — can't enumerate statically; skip)
  - `/chat`
- `vercel.json` already had `/ads.txt` → `/ads.txt` before SPA catch-all.
  Added the same for `/sitemap.xml`.
- `dist/build.md` documents the production bundle flow including the
  `cp ads.txt dist/public/ads.txt` step. Extended with sitemap.
- Canonical domain: `https://www.lilidm.com/` (apex redirects to www).

## Tasks

- [x] 0. Create `sitemap.xml` at project root with all static routes
- [x] 1. Add `/sitemap.xml` route to `vercel.json` (before SPA catch-all)
- [x] 2. Update `dist/build.md`:
      - Added `cp sitemap.xml dist/public/sitemap.xml` to bundle step 3
      - Added new section explaining sitemap.xml (parallel to ads.txt section)
      - Updated git add to include `sitemap.xml`
- [x] 3. Validate sitemap.xml is well-formed XML (`xmllint --noout`) ✅
- [x] 4. Verify vercel.json route order (JSON valid, /sitemap.xml before catch-all) ✅

## Notes

- Dynamic route `/content/:slug` is intentionally omitted — slugs come from
  Supabase at runtime; can't be enumerated statically. (Could add a dynamic
  sitemap generator later if needed.)
- `lastmod` set to current date (2026-07-25); update manually when content
  changes significantly, or wire up a build-time generator later.
- Sitemap uses `https://www.lilidm.com/` (www) since apex redirects to www.
- No rebuild/redeploy was done — sitemap + vercel route are config-only.
  Next `dx bundle` will pick them up via the documented manual-copy step.
