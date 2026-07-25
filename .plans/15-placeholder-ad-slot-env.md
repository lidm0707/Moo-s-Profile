# Plan: Make `PLACEHOLDER_AD_SLOT` env-overridable

## Goal

Make the AdSense slot sentinel `PLACEHOLDER_AD_SLOT` configurable via env var
(parallel to `PLACEHOLDER_CLIENT_ID`), and document it in `.env.sample` per
user request.

## Background

- Currently `PLACEHOLDER_AD_SLOT` was a hard-coded `pub const` in
  `src/components/adsense.rs:36` (`"0000000000"`).
- The companion `PLACEHOLDER_CLIENT_ID` was already env-overridable via
  `build.rs:34-35` (default `ca-pub-0000000000000000`).
- User asked: "PLACEHOLDER_AD_SLOT to sample" → make it env-overridable AND
  document in `.env.sample`.
- Reverses part of plan 14's decision to hide placeholder sentinels from
  `.env.sample`. User wants them exposed.

## Tasks

- [x] 0. `build.rs`: read `PLACEHOLDER_AD_SLOT` env var (default `"0000000000"`),
      inject via `cargo:rustc-env`. Also added parallel `PLACEHOLDER_CLIENT_ID`
      documentation to `.env.sample` for consistency.
- [x] 1. `adsense.rs`: change `pub const PLACEHOLDER_AD_SLOT` to use `env!()`
- [x] 2. `.env.sample`: add commented-out `PLACEHOLDER_AD_SLOT` + `PLACEHOLDER_CLIENT_ID`
- [x] 3. `cargo check` (default) — clean
- [x] 4. `cargo clippy` (default) — clean
- [x] 5. `cargo check --features ads` — clean (recompiled adsense.rs)
- [x] 6. `cargo clippy --features ads` — clean

## Notes

- `PLACEHOLDER_AD_SLOT` is intentionally NOT feature-gated (callers compare
  against it without importing the `ads` feature). `env!()` works for all
  feature combos because `build.rs` always runs and sets the env var.
- Default `"0000000000"` preserved so existing behavior is unchanged.
- Documented as commented-out optional override in `.env.sample` (users
  shouldn't normally change it).
- Also added `PLACEHOLDER_CLIENT_ID` to `.env.sample` for parallel
  documentation (it was already env-overridable in build.rs but undocumented).
- Reverses plan 14 note: "PLACEHOLDER_CLIENT_ID env var is intentionally NOT
  exposed — it's an internal implementation detail that users shouldn't
  override." User explicitly wants these exposed.
