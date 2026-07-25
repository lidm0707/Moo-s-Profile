# Plan 12: Google AdSense Integration for Dioxus 0.7+

**Created:** 2026-07-24
**Status:** Done

## Goal

Integrate Google AdSense into the CSR (web-only) Dioxus 0.7 app: load the
AdSense script exactly once, render reusable ad slots, initialize them safely
on mount (and after SPA navigation), and never crash if the script/ad-blocker
fails. No new crate deps; config via the existing `build.rs` + `env!` pattern.

## Context / Constraints

- Project is **CSR-only** (`Cargo.toml` has only `web` feature, no fullstack).
  SSR/hydration is not active, but all DOM/JS calls are `#[cfg(target_arch = "wasm32")]` guarded for robustness.
- Config pattern: `build.rs` reads `.env` via `dotenvy` → `cargo:rustc-env=` → runtime `env!("VAR")` (see `routes/mod.rs` `env!("APP_MODE")`).
- `.env` is gitignored and not accessible to the agent → `build.rs` must tolerate a missing `ADSENSE_CLIENT_ID` with a placeholder default so builds never break.
- AdSense queue semantics: `(window.adsbygoogle = window.adsbygoogle || []).push({})` **queues** pushes before the lib loads, so "retry if delayed" (Phase 3) is handled for free — no polling loop needed (lean code rule).
- `document::Script` (Dioxus 0.7) renders a `<script>` in `<head>` and dedupes by `src` → global script is injected exactly once even across SPA nav (App never remounts).
- `Adsense` component `use_effect` runs once per mount (reads no signals); on navigation the component remounts → initializes only the newly created `<ins>`. No double-init.
- Project rules: no hardcoded values (use consts), no `pub use _`, `#[allow(dead_code)]` forbidden (use `todo!()`/disabled-render instead), `cargo check && cargo clippy` after each task.

## Tasks

- [x] 0. Config plumbing: add `ADSENSE_CLIENT_ID` read to `build.rs` with a
      `ca-pub-0000000000000000` placeholder default (never `expect/panic`).
      Add Cargo feature `ads = []` (default-OFF, dev-safe) in `Cargo.toml`.
- [x] 1. Create `src/components/adsense.rs`:
      - Config consts section: `ADSENSE_CLIENT_ID` (from `env!`), `DEFAULT_AD_FORMAT`,
        `DEFAULT_RESPONSIVE`, test-mode flag.
      - `#[wasm_bindgen(inline_js)]` extern: `push_adsense()` = `try { (window.adsbygoogle = window.adsbygoogle || []).push({}); } catch(e) {}`.
      - `#[component] Adsense { ad_slot, ad_format, responsive, style, class }`
        rendering `<ins class="adsbygoogle" ...>` + a `use_effect` that calls
        `push_adsense()` once on mount (wasm32-guarded; no-op + dev warning otherwise).
      - When `ads` feature is OFF, component renders `None` (so all call sites compile without cfg).
      - Export in `src/components/mod.rs`: `pub mod adsense; pub use adsense::Adsense;`

      NOTE (0.7 discovery): `Element` in 0.7 is `Result<VNode, RenderError>`, NOT
      `Option<VNode>` — "render nothing" is `rsx! {}` (early return), not `return None;`.
      Also `use_effect`/cfg-gating *inside* the `#[component]` body confuses dead-code
      analysis, so the ads-on logic lives in a top-level `#[cfg(feature="ads")] fn
      render_adsense(...)` that the component delegates to. All warnings resolve once
      Task 3 actually instantiates `<Adsense/>` (the whole chain is dead code until then).
- [x] 2. Global loader in `main.rs` `App()`: add `document::Script` for the
      AdSense lib, `src` built from `ADSENSE_CLIENT_ID`, `crossorigin:"anonymous"`,
      gated by `#[cfg(feature = "ads")]` AND a non-empty/placeholder check (skip if still placeholder → avoids a broken request in dev).
      (Implementation: `pub(crate) fn adsense_script_src() -> Option<String>` in
      adsense.rs returns the URL or None; `main.rs` does `if let Some(src) = ...`.
      `document::Script` dedupes by src in 0.7 → injected exactly once across SPA nav.)
- [x] 3. Demo: place one `Adsense` ad slot on the Content listing page
      (`src/features/content/content_grouped.rs`), gated by `#[cfg(feature = "ads")]`, to
      demonstrate integration on a natural page without a throwaway route.
      (Instantiated UNCONDITIONALLY — the component self-gates — which is what makes
      the whole chain live and warning-free in both build modes. `DEMO_AD_SLOT`
      const = "1234567890" placeholder.)
- [x] 4. Docs: `docs/adsense.md` — Google AdSense setup steps, env var, how to
      add a new ad, dev behavior (feature flag), common errors (ad blocker,
      duplicate push), production notes.
- [x] 5. Validate: `cargo check` && `cargo clippy` (fix any diagnostics from this work only).
      (FINAL: `cargo check` + `cargo clippy` pass with ZERO warnings in BOTH
      default (ads off) and `--features ads` builds. `diagnostics` on adsense.rs clean.)

## Follow-up (2026-07-24)

- [x] 6. Move `PLACEHOLDER_CLIENT_ID` to env (project "no hardcode" rule).
      - `build.rs`: read `PLACEHOLDER_CLIENT_ID` (default `ca-pub-0000000000000000`),
        use it as the fallback for `ADSENSE_CLIENT_ID`, inject BOTH via `cargo:rustc-env=`.
        The literal now appears exactly once (the fallback default in build.rs).
      - `adsense.rs`: `const PLACEHOLDER_CLIENT_ID: &str = env!("PLACEHOLDER_CLIENT_ID");`
      - `docs/adsense.md`: document optional `PLACEHOLDER_CLIENT_ID` env var.
      - Validate: `cargo clippy` clean in both feature modes.
- [x] 7. Add `.env.example` (documented, committable template for all env vars
      read by `build.rs`: APP_MODE, SUPABASE_URL, SUPABASE_ANON_KEY,
      ADSENSE_CLIENT_ID, PLACEHOLDER_CLIENT_ID). Confirmed NOT gitignored.
      `docs/adsense.md` points to it.
- [x] 8. Add "how to use AdSense" docs:
      - `README.md`: new `## How to Use Google AdSense` quick-start (publisher id,
        `--features ads`, `Adsense` component snippet) + `## Environment Variables`
        section; updated Project Structure to list `docs/`, `src/components/`,
        `.env.example`, `ads` feature.
      - `docs/adsense.md`: added a `## Quick start (TL;DR)` block at the top.
      - Validate: `cargo check` clean.

## Round 2 — Optional Enhancements (2026-07-24)

The optional features were scoped out in round 1; user wants them implemented.

- [x] 9. `AdFormat` enum (project rule "prefer enums over hard-coded values")
      replacing the `Option<String>` ad_format. Variants Auto/Horiz/Rect/Vertical
      → `data-ad-format` strings.
      **REVERSED:** a 4-variant enum triggers "variants never constructed" in a
      binary crate (everything is internal), and `#[allow(dead_code)]` is forbidden
      by the project rules. Kept `ad_format: Option<String>` with a `DEFAULT_AD_FORMAT`
      const default instead — no magic strings in logic, zero dead-code warnings.
- [x] 10. Lazy-load via IntersectionObserver: `lazy: Option<bool>` prop
      (default true). On `onmounted`, observe the `<ins>` element and only call
      `push` when it enters the viewport (rootMargin prefetch). Graceful fallback
      to immediate push if IntersectionObserver unsupported or element unavailable.
      API verified: `evt.downcast::<web_sys::Element>()` (MountedData, web-only).
      Implemented via inline_js `lazy_push_adsense(el, rootMargin)`.
- [x] 11. Theme-aware rendering: optional `dark_mode: Option<bool>` prop (NOT
      context — `try_context` doesn't exist in 0.7 and `use_context` panics if
      absent) → adds `adsense-wrap-dark`/`-light` class to a wrapper. Added CSS in
      `assets/main.css` (min-height placeholder to avoid CLS).
- [x] 12. Update demo call site + docs; validate `cargo clippy` both modes.
      Demo passes `dark_mode: Some(dark_mode())`. `docs/adsense.md` updated
      (props, behavior, enhancement status). `cargo clippy` clean both modes.

Notes:
- NOT implementing analytics hooks: AdSense renders into a cross-origin iframe,
  so reliable click/impression detection isn't possible from the host page. A
  heuristic `on_impression` on mount would be misleading. Documented as why-not.
- NOT implementing "automatic late-script retry": AdSense's push queue already
  handles late script loading, so it's redundant.

## Notes

- Choosing `ads` default-OFF: prevents real ad requests during `dx serve` dev and
  honors the "feature flag to disable ads during development" optional enhancement.
- Config lives as a clearly-sectioned const block inside `adsense.rs` rather than a
  new top-level module — honors the project "lean code / no over-engineer" rule.
  If it grows, extract to `src/config/adsense.rs` later.
- Not implemented from optional list (out of scope, documented as future): lazy-load
  via IntersectionObserver, dark/light auto-theme, analytics hooks. IntersectionObserver
  would reduce impressions but adds complexity; AdSense already lazy-loads internally.
