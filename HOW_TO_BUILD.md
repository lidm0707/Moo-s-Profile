# Build & Deploy Manual

Moo's Profile is a Dioxus 0.7 (Rust → WASM) single-page app. The production
site is a static bundle in `dist/public`, deployed to Vercel **from the local
machine** via the Vercel CLI. Vercel never builds this project — its build
image has no Rust toolchain or `dioxus-cli`.

```
make build            # local: snapshot + sitemap + dx bundle → dist/public
vercel deploy dist --prod   # upload only dist/ + vercel.json
```

## 1. Prerequisites

| Tool | Install | Notes |
|---|---|---|
| Rust (stable) | `rustup` | WASM target added automatically by `dx` |
| dioxus-cli | `cargo install dioxus-cli` | provides the `dx` command |
| Node.js ≥ 18 | — | for the `scripts/*.mjs` helpers |
| Vercel CLI | `npm i -g vercel` | one-time `vercel login` + `vercel link` |

## 2. Environment

Copy the template and fill in real values:

```sh
cp .env.sample .env
```

Required variables (both locally and at compile time — `build.rs` reads
`.env` via `dotenvy` and the build fails without them):

- `SUPABASE_URL`
- `SUPABASE_ANON_KEY`
- `APP_MODE` — `development` or `production`

Optional:

- `ADSENSE_CLIENT_ID` — overrides the ID parsed from `index.html`
- `SITEMAP_BASE_URL` — defaults to `https://www.lilidm.com`

`.env` is gitignored. **Never commit it, and never commit the Supabase
service role key.**

## 3. Everyday commands

```sh
make help      # list all targets
make dev       # dx serve — dev server with hot reload
make check     # cargo check
make test      # cargo check + clippy -D warnings
make clean     # remove dist/public and target/
```

## 4. Production build — `make build`

One command runs the whole pipeline:

```sh
make build            # alias: make prepare
```

Steps in order:

1. `clean` — delete old `dist/public` and `target/`
2. `snapshot` — fetch all Supabase content into `assets/content_snapshot.json`
3. `sitemap` — regenerate `sitemap.xml` from routes + Supabase content
4. `dx bundle --release --out-dir ./dist` — release WASM bundle
5. Compose `dist/public`:
   - copy `ads.txt`, `robots.txt`, `sitemap.xml` (served un-hashed at `/<name>`)
   - `assets/profile.jpg` → `og-image.jpg`
   - `content_snapshot.json` into the bundle
   - `generate-content-meta.mjs` — per-content meta pages under `dist/public/content/`

`dist/` is gitignored. The artifact lives only on disk; shipping is the
deploy step's job.

## 5. Deploy to Vercel

First time only: `vercel login` and `vercel link` (creates `.vercel/`, also
gitignored).

Every deploy:

```sh
vercel deploy dist --prod
```

- `.vercelignore` makes the CLI upload **only** `dist/` and `vercel.json`,
  so pushes are small even though the repo root is huge.
- `vercel.json` maps routes: `/assets/*`, `/content/*`, the four root
  static files, and everything else → `dist/public/index.html` (SPA
  fallback).

The GitHub connection on the Vercel project is intentionally unused — do
not rely on push-to-deploy.

## 6. Content workflow

Site content lives in Supabase, not in the repo. `make build` snapshots it
into the bundle, so a redeploy is required after editing content in
Supabase. To refresh just the snapshot during development:

```sh
make snapshot
```

## 7. Troubleshooting

| Symptom | Cause / fix |
|---|---|
| Build fails with missing env | `.env` missing or missing `SUPABASE_URL` / `SUPABASE_ANON_KEY` |
| `dx: command not found` | `cargo install dioxus-cli` |
| Site 404 after deploy | deployed something other than `dist/`, or `dist/public` is empty — rerun `make build` |
| Stale content on the site | old snapshot — run `make build` (not just `vercel deploy`) |
| Clippy failure in `make test` | fix warnings; CI parity is `cargo clippy -- -D warnings` |
