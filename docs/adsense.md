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

Add the publisher ID to your `.env` (this file is gitignored and read by
`build.rs` at compile time). A documented template lives in
[`.env.example`](../.env.example) — copy it to start:

```sh
cp .env.example .env
```

```dotenv
# .env  (existing APP_MODE / SUPABASE_* vars live here too)
ADSENSE_CLIENT_ID=ca-pub-XXXXXXXXXXXXXXXX

# Optional: override the sentinel that marks an unset publisher ID.
# Defaults to ca-pub-0000000000000000. Only set this if you need a custom
# placeholder (e.g. a test publisher id).
# PLACEHOLDER_CLIENT_ID=ca-pub-0000000000000000
```

`build.rs` injects both via `cargo:rustc-env=`, so the app reads them at runtime
with `env!()` (`ADSENSE_CLIENT_ID`, `PLACEHOLDER_CLIENT_ID`).

> If `ADSENSE_CLIENT_ID` is unset, the build still succeeds using the
> `PLACEHOLDER_CLIENT_ID` value. The loader and the `Adsense` component both
> detect when the publisher ID equals the placeholder and render nothing (with a
> dev console warning), so the site keeps working.

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
  initialized. The global script is **not** re-injected (Dioxus dedupes
  `document::Script` by `src`, and `App` never remounts).

## 5. Local development behavior

- **Default (`dx serve`):** ads disabled. No network requests to AdSense, no
  blank ad boxes. Safe for normal dev work.
- **`dx serve --features ads` with placeholder ID:** ad slots render nothing and
  the console warns `[adsense] ADSENSE_CLIENT_ID is unset; ad slot not rendered.`
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
