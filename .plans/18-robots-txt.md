# Plan: Add robots.txt

## Goal

Add a `robots.txt` at the project root following the same deploy pattern as
`ads.txt` and `sitemap.xml` (root file → `vercel.json` route before SPA
catch-all → manual `cp` into `dist/public/` after `dx bundle`).

## Background

- Site is a public personal profile + blog. Nothing to hide from crawlers.
- AdSense is integrated (`index.html` has the AdSense script, `ads.txt` set);
  AdSense's `Mediapartners-Google` crawler is covered by the permissive
  `User-agent: *` rule.
- Canonical origin: `https://www.lilidm.com/` (apex redirects to www).
- `sitemap.xml` already exists; referencing it from `robots.txt` is the
  standard way for crawlers to discover it without manual Search Console
  submission.

## Decision: no Disallow rules

- `/chat` is harmless to crawl (in the sitemap at priority 0.5 already).
- `/content/:slug` now redirects client-side to `/content?tag_id=…&content_id=…`
  — Google handles that fine; no need to disallow.
- No admin/auth-gated routes exist in the SPA.

## Tasks

- [x] 0. Create `robots.txt` at project root
      - `User-agent: *` / `Allow: /`
      - `Sitemap: https://www.lilidm.com/sitemap.xml`
- [x] 1. Add `/robots.txt` route to `vercel.json` (before SPA catch-all,
      between `/ads.txt` and `/sitemap.xml` to keep alphabetical-ish order)
- [x] 2. Update `dist/build.md`:
      - Step 3 copies `robots.txt` alongside `ads.txt` and `sitemap.xml`
      - Step 4 `git add` includes `robots.txt`
      - New "robots.txt (crawler directives)" section parallel to ads.txt /
        sitemap.xml sections
- [x] 3. Verify `vercel.json` is still valid JSON

## Notes

- Static file, no generator needed (unlike `sitemap.xml`).
- If staging/preview domains ever need different directives, add a second
  `robots.txt` per environment and switch in the deploy step. Not needed today.
- Did not rebuild/redeploy — config + static-file only. Next `dx bundle`
  picks it up via the documented manual-copy step.
