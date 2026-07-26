# Build & deploy

Vercel deploys the committed `dist/` directly (`vercel.json` has
`"outputDirectory": "dist/public"` and no build command), so every deploy is a
committed bundle.

## Production bundle

```sh
# 1. clean previous output (avoids stale hashed assets accumulating)
rm -rf dist/public

# 2. bundle (pre-existing wasm-opt SIGABRT toolchain quirk; bundle still completes)
dx bundle --release --out-dir ./dist

# 3. copy root-level static files that Dioxus must NOT hash
#    (ads.txt must be served at exactly /ads.txt for AdSense;
#     robots.txt must be served at exactly /robots.txt for SEO;
#     sitemap.xml must be served at exactly /sitemap.xml for SEO;
#     the Dioxus.toml `static` array hashes filenames, so we copy manually)
cp ads.txt dist/public/ads.txt
cp robots.txt dist/public/robots.txt
# refresh sitemap.xml with the latest published content from Supabase
node scripts/update-sitemap.mjs
cp sitemap.xml dist/public/sitemap.xml

# 4. commit & push to deploy
git add dist/ ads.txt robots.txt sitemap.xml
git commit -m "build: rebuild dist"
git push
```

## Ads.txt (AdSense seller verification)

AdSense requires `https://<your-domain>/ads.txt` to verify you're an authorized
seller of your ad inventory. Without it, AdSense shows "ads.txt status not
found" and may stop serving ads.

- **Source of truth:** [`ads.txt`](../ads.txt) in the project root.
- **Format:** `google.com, pub-XXXXXXXXXXXXXXXX, DIRECT, f08c47fec0942fa0`
  where the publisher id is the number part only (no `ca-` prefix) and
  `f08c47fec0942fa0` is Google's standard TAG-ID.
- **Deployed path:** `dist/public/ads.txt` → served at `/ads.txt`.
- **Vercel routing:** `vercel.json` has an explicit `/ads.txt` route BEFORE the
  SPA catch-all so the file is served instead of `index.html`.
- **Rebuild note:** step 3 above copies it after `dx bundle` — `rm -rf
  dist/public` wipes it, so always re-copy.

## sitemap.xml (SEO crawler discovery)

A static sitemap at `https://<your-domain>/sitemap.xml` helps Google and other
search engines discover the app's routes. This is a client-rendered SPA, so
crawlers may not find all routes by following links alone — the sitemap lists
them explicitly.

- **Source of truth:** [`sitemap.xml`](../sitemap.xml) in the project root.
- **Routes included:** `/`, `/interests`, `/work-history`, `/content`, `/chat`
  (static) plus one `/content/<slug>` per published row in the `content` table
  (dynamic — fetched from Supabase by the generator below).
- **Canonical domain:** `https://www.lilidm.com/` (apex redirects to www).
- **Deployed path:** `dist/public/sitemap.xml` → served at `/sitemap.xml`.
- **Vercel routing:** `vercel.json` has an explicit `/sitemap.xml` route BEFORE
  the SPA catch-all so the file is served instead of `index.html`.
- **Rebuild note:** step 3 above copies it after `dx bundle` — `rm -rf
  dist/public` wipes it, so always re-copy.
- **Generator:** [`scripts/update-sitemap.mjs`](../scripts/update-sitemap.mjs)
  rebuilds `sitemap.xml` at the root from the published content in Supabase
  (reads `SUPABASE_URL` / `SUPABASE_ANON_KEY` from `.env`). Run it before the
  `cp` above so the deployed sitemap reflects current content. Tag-filtered
  URLs (`/content?tag_id=…`) are intentionally omitted — those are SPA filter
  state, not distinct resources; individual posts are already listed by slug.

## robots.txt (crawler directives)

A `robots.txt` at `https://<your-domain>/robots.txt` tells crawlers what they
may crawl. Combined with the sitemap reference, it's the standard crawler
entry point — Google Search Console and Bing Webmaster Tools both fetch it
first.

- **Source of truth:** [`robots.txt`](../robots.txt) in the project root.
- **Directives:** `User-agent: *` / `Allow: /` (no restrictions — public
  profile + blog) plus a `Sitemap:` line pointing at the canonical origin.
- **Canonical domain:** `https://www.lilidm.com/` (apex redirects to www).
- **Deployed path:** `dist/public/robots.txt` → served at `/robots.txt`.
- **Vercel routing:** `vercel.json` has an explicit `/robots.txt` route BEFORE
  the SPA catch-all so the file is served instead of `index.html`.
- **Rebuild note:** step 3 above copies it after `dx bundle` — `rm -rf
  dist/public` wipes it, so always re-copy.
- **Editing:** text-only file, edit in place. No `lastmod` to bump.
