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
#     sitemap.xml must be served at exactly /sitemap.xml for SEO;
#     the Dioxus.toml `static` array hashes filenames, so we copy manually)
cp ads.txt dist/public/ads.txt
cp sitemap.xml dist/public/sitemap.xml

# 4. commit & push to deploy
git add dist/ ads.txt sitemap.xml
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
  (the dynamic `/content/:slug` route is omitted — slugs come from Supabase at
  runtime and can't be enumerated statically).
- **Canonical domain:** `https://www.lilidm.com/` (apex redirects to www).
- **Deployed path:** `dist/public/sitemap.xml` → served at `/sitemap.xml`.
- **Vercel routing:** `vercel.json` has an explicit `/sitemap.xml` route BEFORE
  the SPA catch-all so the file is served instead of `index.html`.
- **Rebuild note:** step 3 above copies it after `dx bundle` — `rm -rf
  dist/public` wipes it, so always re-copy.
- **Updating `lastmod`:** edit `sitemap.xml` at the root and bump the
  `<lastmod>` dates when content changes significantly, then rebuild.
