# Link Previews for a Dioxus SPA — How This Project Does It

A short learning doc: why shared links showed no image card, and how we generate
per-article previews from Supabase content at build time.

---

## 1. The problem

Sharing `https://www.lilidm.com/content/Rust_is_hard` in Discord / LINE / X showed
a blank card.

**Why:** the site is a client-rendered SPA (Dioxus → Wasm). Link unfurlers
(crawlers) never execute JavaScript or Wasm — they fetch the URL and read only the
**static HTML head**. Our `index.html` had:

- no `og:*` / `twitter:*` meta tags at all
- one shared head for every route (the app renders content *after* load, in Wasm)

So crawlers saw nothing to preview, no matter which route was shared.

## 2. Key rule

> Everything a crawler needs must exist in the raw HTML served for that URL.

Meta tags injected at runtime (JS/Wasm, `document.createElement`, etc.) do not
count. Two ways to satisfy the rule:

| Approach | How | Cost |
|---|---|---|
| SSR / fullstack | server renders meta per request | Dioxus fullstack + a server |
| **Pre-render at build** | generate static HTML per route at build time | a build script — no server needed |

This project has no server (static files on Vercel), so we pre-render at build.

## 3. What we already had

`make build` already fetched every published slug from Supabase to build
`sitemap.xml` (`scripts/update-sitemap.mjs`). The same data can produce preview
pages — no new data source needed.

Supabase REST read used by both scripts:

```
GET {SUPABASE_URL}/rest/v1/content?select=slug,title,body&status=eq.published
headers: apikey: <anon key>, Authorization: Bearer <anon key>
```

## 4. The pieces

### a) Global OG tags — `index.html`

Static tags in the head that every route inherits:

```html
<title>Moo's Profile — Web Content & Chatbot</title>
<meta property="og:title" content="...">
<meta property="og:description" content="...">
<meta property="og:image" content="https://www.lilidm.com/og-image.jpg">
<meta name="twitter:card" content="summary_large_image">
```

Rules learned:
- `og:image` must be an **absolute URL**
- the image file must live at an **un-hashed** path (Dioxus hashes asset
  filenames; an OG image can't be `og-image-dxh4f2.jpg` that changes per build)
  → `make build` copies `assets/profile.jpg` to `dist/public/og-image.jpg`

### b) Per-content pages — `scripts/generate-content-meta.mjs`

Runs **after** `dx bundle` (needs files the bundle produces):

1. Read root `index.html` as the template (has the generic OG block).
2. Read `dist/public/index.html` and extract the **hashed** asset tags
   (`main-*.css`, preload, `my_profile-*.js`) with a regex — these names change
   every build and are only known after bundling.
3. Fetch published content from Supabase.
4. For each slug: swap `<title>`, description, `og:*`, `twitter:*` in the
   template; description = first ~160 chars of `body` (HTML tags and markdown
   stripped, `…` appended).
5. Write `dist/public/content/<slug>/index.html`.

Gotcha hit: Vercel-file paths look like `/./assets/...`, and the JS tag is
`<script type="module" ...>` — the first regex missed it. Generic tag matching
fixed it:

```
/<link[^>]*assets\/(?:main|my_profile)[^>]*>|<script[^>]*assets\/my_profile[^>]*><\/script>/g
```

Why the pages still work for humans: they load the same Wasm app; the Dioxus
router reads the URL and renders the article. Bots stop at the meta tags, people
get the SPA.

### c) Serving the files — `vercel.json`

Static files must win over the SPA catch-all. Explicit route before `/(.*)`:

```json
{ "src": "/assets/(.*)", "dest": "/assets/$1" },
{ "src": "/content/(.*)", "dest": "/content/$1" },
{ "src": "/(.*)", "dest": "/index.html" }
```

### d) Pipeline — `Makefile`

```make
build: clean sitemap
	dx bundle --release --out-dir ./dist          # hashed assets now exist
	cp ads.txt robots.txt sitemap.xml dist/public/
	cp assets/profile.jpg dist/public/og-image.jpg
	node scripts/generate-content-meta.mjs        # per-slug HTML pages
```

`rm -rf dist/public` + `rm -rf target` run first (`clean`), so every build is
from scratch and stale hashed assets/slug pages can't accumulate.

## 5. Debugging checklist

- `curl https://site/content/<slug>` → check `<title>` / `og:` tags are in the
  raw HTML (if not: build order wrong, script didn't run, or route rewrote to
  the root index).
- Preview still stale on Discord → unfurlers **cache per URL**. Re-post with a
  throwaway query param (`?v=2`) or wait for cache expiry.
- Image missing → must be absolute URL, publicly reachable, un-hashed, and
  usually ≥ 1200×630 works best for `summary_large_image` (we declare 800×800
  with `summary_large_image`; cards still render, image is letterboxed).

## 6. Takeaways

1. Crawlers read raw HTML only — SPA meta must be static.
2. Per-route previews without a server = pre-render files at build time.
3. Reuse build-time data (sitemap fetch → preview pages).
4. OG images need stable un-hashed URLs.
5. Static files need explicit routes to beat a SPA catch-all.
6. Unfurlers cache aggressively; test with fresh query params.
