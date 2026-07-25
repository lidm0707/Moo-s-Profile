# Google AdSense Integration

This project integrates [Google AdSense](https://www.google.com/adsense/) behind
a Cargo feature flag so ads are **off by default** (no real ad requests during
`dx serve`) and can be enabled explicitly for preview/production builds.

Everything lives in [`src/components/adsense.rs`](../src/components/adsense.rs).

## Quick start (TL;DR)

```bash
# 1. configure your publisher id (placeholder disables ads when unset)
cp .env.example .env   # then edit: ADSENSE_CLIENT_ID=ca-pub-XXXXXXXXXXXXXXXX

# 2. run/build WITH the ads feature

dx serve  --features ads          # preview
dx build --release --features ads # production
```

```rust
// 3. drop a slot anywhere
use crate::components::Adsense;
rsx! { Adsense { ad_slot: "1234567890".to_string() } }   // your AdSense slot id
```

Done. Without `--features ads`, every `Adsense` renders nothing and no AdSense
script is loaded. The sections below cover full setup, options, and troubleshooting.

---

## 1. Required Google AdSense setup (one-time)

1. Create / sign in to your AdSense account.
2. Add your site (`https://<your-domain>`) and complete verification.
3. Copy your **publisher ID**, formatted `ca-pub-XXXXXXXXXXXXXXXX` (found in
   AdSense → Account → Account information, or in the snippet `data-ad-client`).
4. Create one or more **ad units** (AdSense → Ads → By ad unit → Display ads) and
   copy each **Slot ID** (the numeric `data-ad-slot`, e.g. `1234567890`).

## 2. Configure the publisher ID

The AdSense library script must live in the **static `index.html`** (project
root) so Google's verification crawler can find it — this is a client-rendered
app, so anything injected at runtime by JS is invisible to the crawler. The
script tag is already in [`index.html`](../index.html):

```html
<script async
  src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-XXXXXXXXXXXXXXXX"
  crossorigin="anonymous"></script>
```

Replace the `ca-pub-XXXXXXXXXXXXXXXX` there with your own publisher ID (from the
AdSense console → Account information). **`index.html` is the single source of
truth for the publisher ID** — `build.rs` parses it at compile time and bakes
the same value into the `Adsense` component's `data-ad-client`, so you do NOT
need to mirror it anywhere else.

### Optional: `.env` override

If you want to override the id from `index.html` (e.g. for a test build), set
`ADSENSE_CLIENT_ID` in `.env`. The build resolves the publisher ID in this
priority order:

1. `ADSENSE_CLIENT_ID` env var (if set and not equal to the placeholder)
2. parsed from `index.html` (`adsbygoogle.js?client=ca-pub-XXXX`)
3. `PLACEHOLDER_CLIENT_ID` (disables ads at runtime)

`.env` is gitignored and read by `build.rs` at compile time; a template lives
in [`.env.example`](../.env.example):

```sh
cp .env.example .env
```

```dotenv
# .env  (existing APP_MODE / SUPABASE_* vars live here too)
# Optional — leave commented to use the id from index.html:
# ADSENSE_CLIENT_ID=ca-pub-XXXXXXXXXXXXXXXX

# Optional: override the sentinel that marks an unset publisher ID.
# Defaults to ca-pub-0000000000000000. Only set this if you need a custom
# placeholder (e.g. a test publisher id).
# PLACEHOLDER_CLIENT_ID=ca-pub-0000000000000000
```

> Site verification only needs the script in `index.html` — it does not depend
> on `.env` or the `ads` feature. Ad *slots* additionally need the `ads` feature
> enabled (see §3); the publisher ID comes from `index.html` automatically.


`build.rs` injects both via `cargo:rustc-env=`, so the app reads them at runtime
with `env!()` (`ADSENSE_CLIENT_ID`, `PLACEHOLDER_CLIENT_ID`).

> If the publisher ID can't be resolved (no `index.html` script tag AND no
> `ADSENSE_CLIENT_ID` env var), the build still succeeds using the
> `PLACEHOLDER_CLIENT_ID` value. The `Adsense` component detects this at runtime
> and renders nothing (with a dev console warning), so the site keeps working.

## 3. Enable ads

Ads are gated behind the `ads` cargo feature (off by default):

```sh
# Preview with ads (uses your ADSENSE_CLIENT_ID from .env)
dx serve --features ads

# Production build with ads
dx build --release --features ads
```

Without `--features ads`, every `Adsense` component renders nothing and the
global AdSense script is never injected.

## 4. Add a new ad

Import the component and drop it anywhere:

```rust
use crate::components::Adsense;

rsx! {
    Adsense {
        ad_slot: "1234567890".to_string(),         // required: AdSense slot id
        // Optional (all have sensible defaults):
        ad_format: Some("auto".to_string()),       // "auto" | "horizontal" | "rectangle" | "vertical"
        responsive: Some(true),                    // data-full-width-responsive
        lazy: Some(true),                          // load when scrolled into view (IntersectionObserver)
        dark_mode: Some(dark_mode()),              // theme-aware wrapper class (pass your theme signal)
        style: Some("display:block".to_string()),  // inline style on the <ins>
        class: Some("my-ad-wrap".to_string()),     // extra class(es) appended to "adsbygoogle"
    }
}
```

Behavior notes:

- The component renders `<div class="adsense-wrap ..."><ins class="adsbygoogle" ...>`.
  When `dark_mode` is `Some(true)`/`Some(false)`, an `adsense-wrap-dark` /
  `adsense-wrap-light` class is added to the wrapper — style these in CSS to match
  the site theme (e.g. a min-height placeholder to prevent layout shift).
- By default ads **lazy-load**: an `IntersectionObserver` watches the `<ins>` and
  calls `(window.adsbygoogle = window.adsbygoogle || []).push({})` only when it
  enters the viewport (prefetch `rootMargin: 200px`). Set `lazy: Some(false)` to
  initialize immediately. If `IntersectionObserver` is unavailable, it falls back
  to immediate init.
- AdSense **queues** pushes before its library finishes loading, so calling push
  early is safe — no retry/polling needed.
- On SPA navigation the component remounts, so only the newly created `<ins>` is
  initialized. The AdSense library loads once from the static `index.html` script
  tag and is never re-fetched on navigation.

## 4b. Site verification

Google must verify your site before serving ads. The verification crawler reads
the **raw HTML**, so the AdSense `<script>` must be in the static `index.html`
(see §2) — runtime injection is invisible to the crawler.

After setting the publisher ID in `index.html`, rebuild and deploy:

```sh
dx bundle --release --out-dir ./dist   # produces dist/ which Vercel deploys
git add index.html dist/
git commit -m "build: add adsense verification script"
git push
```

Then in the AdSense console, request verification of `https://<your-domain>`.
If it still fails, confirm the script tag with your `client=ca-pub-...` is
visible in the deployed page's HTML (View Source, *not* the Elements panel).

## 4c. Consent banner (Google CMP / Funding Choices)

GDPR/CCPA consent is handled by **Google's Consent Management Platform
(Funding Choices)**. It shows a banner with 2 buttons — **Consent** and
**Manage options** — and gates ad personalization accordingly. The message
itself is created in the **AdSense console**, not in code.

### How it works

The CMP loader is in the static [`index.html`](../index.html) and runs
**before** the AdSense library so consent is established first:

```html
<!-- 1) Funding Choices CMP loader (place BEFORE adsbygoogle) -->
<script async
  src="https://fundingchoicesmessages.google.com/i/pub-XXXXXXXXXXXXXXXX?ers=1"></script>
<script>(function() {
    function signalGooglefcPresent() {
        if (!window.frames['googlefcPresent']) {
            if (document.body) {
                const iframe = document.createElement('iframe');
                iframe.style = 'width: 0; height: 0; border: none; z-index: -1000; left: -1000px; top: -1000px;';
                iframe.style.display = 'none';
                iframe.name = 'googlefcPresent';
                document.body.appendChild(iframe);
            } else {
                setTimeout(signalGooglefcPresent, 0);
            }
        }
    }
    signalGooglefcPresent();
})();</script>

<!-- 2) AdSense library -->
<script async
  src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-XXXXXXXXXXXXXXXX"
  crossorigin="anonymous"></script>
```

The publisher id for the CMP URL is the **number part only** of your AdSense
client id (strip the `ca-` prefix). For example, if your AdSense client id is
`ca-pub-3526470154848781`, the CMP URL uses `pub-3526470154848781`.

### Create the consent message (one-time, in AdSense console)

1. Sign in to [AdSense](https://www.google.com/adsense/).
2. Go to **Privacy & messaging → GDPR** and click **Create message**.
3. Select your site and choose a message type that gives **2 choices**:
   * a primary **Consent** button, and
   * a secondary **Manage options** button.
4. Edit the text and button labels, then **Publish**.
5. Repeat under **Privacy & messaging → CCPA** if you serve California users.

Once published, the CMP loader in `index.html` will display the banner and
AdSense will automatically respect the user's choices (personalized vs.
non-personalized ads). No code change is needed for the banner content.

### Reusing on another site

Only the two `<script>` tags in `index.html` change:

1. Replace `pub-XXXXXXXXXXXXXXXX` in the CMP loader URL with the new site's
   publisher id (number part, no `ca-`).
2. Replace `ca-pub-XXXXXXXXXXXXXXXX` in the AdSense library URL and mirror the
   same id in `.env` as `ADSENSE_CLIENT_ID`.
3. Publish a new GDPR message in the AdSense console for the new domain.

## 4d. ads.txt (seller verification)

AdSense requires `https://<your-domain>/ads.txt` to verify you're an authorized
seller of your ad inventory. Without it, AdSense shows **"ads.txt status not
found"** in the console and may stop serving ads (or serve unpaid PSA).

This project ships an `ads.txt` at the repo root ([`ads.txt`](../ads.txt)):

```txt
google.com, pub-XXXXXXXXXXXXXXXX, DIRECT, f08c47fec0942fa0
```

Fields:

- `google.com` — the ad system domain.
- `pub-XXXXXXXXXXXXXXXX` — your publisher id, **number part only** (no `ca-`
  prefix). Same id as the CMP loader URL.
- `DIRECT` — you sell inventory directly to Google (vs. `RESELLER`).
- `f08c47fec0942fa0` — Google's standard TAG-ID, same for all AdSense publishers.

### Why it isn't in `Dioxus.toml` `static`

The `Dioxus.toml` `[web.resource] static = [...]` array **hashes filenames**
(`bg_profile.jpg` → `bg_profile-dxh8ce2f1df1c948458.jpg` under `assets/`).
AdSense requires the file at exactly `/ads.txt` (root, no hash, no prefix), so
the static-array mechanism can't be used. Instead:

- **Source of truth:** `ads.txt` in the repo root.
- **Deployed path:** `dist/public/ads.txt` (copied after `dx bundle` — see
  [`dist/build.md`](../dist/build.md) step 3).
- **Vercel routing:** `vercel.json` lists `/ads.txt` BEFORE the SPA catch-all
  (`/(.*)` → `/index.html`) so the file is served instead of the SPA shell.

### Verify it's live

After deploying, check `https://<your-domain>/ads.txt` in a browser — you
should see the plain-text line above (not the app's HTML). Then in AdSense →
**Ads → Summary**, the "ads.txt" warning should clear within ~24–48h.

### Reusing on another site

Edit `ads.txt` and replace `pub-XXXXXXXXXXXXXXXX` with the new site's publisher
id (number part only). Re-run the build (including the `cp ads.txt
 dist/public/ads.txt` step) and deploy.

## 5. Local development behavior

- The AdSense library loads from the static `index.html` script tag in **every**
  build (dev included) — this is required for site verification. It makes one
  fetch to Google but shows no ads unless a slot is rendered.
- Ad **slots** only render with the `ads` feature (`dx serve --features ads`)
  and a non-placeholder `ADSENSE_CLIENT_ID` in `.env`.
- **`dx serve --features ads` with placeholder ID:** slots render nothing and the
  console warns `[adsense] ADSENSE_CLIENT_ID is unset; ad slot not rendered.`
- **`dx serve --features ads` with real ID:** real AdSense loads. Note Google
  only serves real ads on approved/verified domains; on `localhost` you may see
  blank ad spaces or PSAs — that's expected and not a bug in this integration.


## 6. Common errors & troubleshooting

| Symptom | Cause / Fix |
| --- | --- |
| Console: `[adsense] ADSENSE_CLIENT_ID is unset` | Set `ADSENSE_CLIENT_ID` in `.env` and rebuild with `--features ads`. |
| Ad slot is blank | Not approved yet, not a verified domain, ad blocker, or running on `localhost`. Approve the domain in AdSense. |
| Console: `TagError: adsbygoogle.push(...) error: ... already initialized` | The same `<ins>` was pushed twice. This integration pushes once per mount via `onmounted`, so it should not occur. If it does, make sure you're not rendering two `Adsense` components that reuse the same slot id simultaneously. |
| Console: ads blocked / failed to load | An ad blocker or network policy blocked the AdSense script. The `push` is wrapped in `try/catch` and fails silently — the app is unaffected. |
| No `<script>` tag for AdSense in `<head>` | Either `ads` feature is off, or the client ID is still the placeholder. See sections 2–3. |

## 7. Production deployment notes

- Build with `dx build --release --features ads` (and set
  `ADSENSE_CLIENT_ID` in the build environment's `.env` / CI secrets).
- Your domain **must** be added and approved in the AdSense console before ads
  serve; unapproved domains show blank spaces.
- For `vercel.json`-based deploys, ensure the build command passes
  `--features ads` (e.g. `dx build --release --features ads`) and that
  `ADSENSE_CLIENT_ID` is present in the deploy environment.
- Google's crawler needs the page to render the `<ins>` markup; since this app is
  client-rendered, AdSense reads the slots client-side. If you later move to
  fullstack/SSR, keep the `push` call gated to run after hydration
  (`onmounted` already satisfies this).

## 8. Enhancement status

Implemented:

- **Lazy-load** via `IntersectionObserver` — the `lazy` prop (default `true`)
  defers initialization until the slot scrolls into view, with a `200px`
  `rootMargin` prefetch. Falls back to immediate init if the API is unavailable.
- **Theme-aware wrapper** — the `dark_mode` prop adds `adsense-wrap-dark` /
  `adsense-wrap-light` classes so the slot blends with the site theme.
- **Responsive sizing** — the `responsive` prop toggles
  `data-full-width-responsive`.
- **Dev feature flag** — ads are off by default (`--features ads` to enable).

Deliberately not implemented:

- **Analytics hooks (ad view/click)** — AdSense renders into a cross-origin
  iframe, so reliable click/impression detection from the host page isn't
  possible. Use Google AdSense's own reporting instead.
- **Automatic late-script retry** — redundant: AdSense's `.push({})` queue
  processes pushes whenever the library loads, so a missing script at init time
  is already handled.
