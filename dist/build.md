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
#     the Dioxus.toml `static` array hashes filenames, so we copy manually)
cp ads.txt dist/public/ads.txt

# 4. commit & push to deploy
git add dist/ ads.txt
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
