# Plan: AdSense Config in `.env.sample`

## Goal

Expose `ADSENSE_CLIENT_ID` (and other build-time env vars) in a committed
`.env.sample` template so the user can copy it to `.env` and set the real
AdSense publisher ID themselves, instead of relying solely on `index.html`
parsing.

## Background

- `build.rs` already supports `ADSENSE_CLIENT_ID` env var as priority #1,
  falling back to parsing `index.html` (priority #2), then `PLACEHOLDER_CLIENT_ID`
  (priority #3). No code change needed in `build.rs`.
- `.env` is gitignored (line 5 of `.gitignore`). `.env.sample` is meant to be
  committed.
- Previous `.env.example` had a footgun: it showed
  `ADSENSE_CLIENT_ID=ca-pub-0000000000000000` verbatim, which equals
  `PLACEHOLDER_CLIENT_ID` and silently disabled ads when copied. (Plan 12
  Round 6 fixed it in-place by commenting the line out, but the commented
  placeholder pattern remained a footgun risk.)
- This plan DELETES `.env.example` and replaces it with `.env.sample` using
  an EMPTY value (`ADSENSE_CLIENT_ID=`) so the build.rs filter rejects it
  and falls back to `index.html`. No footgun.
- All useful content from `.env.example` (Supabase warnings, service role key
  note) is preserved in `.env.sample`.

## Tasks

- [x] 0. Create `.env.sample` with `ADSENSE_CLIENT_ID` + other required env vars
- [x] 1. Verify `.env.sample` is NOT gitignored (only `.env` should be)
- [x] 2. `cargo check` (default features) — clean
- [x] 3. `cargo clippy` (default features) — clean
- [x] 4. `cargo check --features ads` — clean
- [x] 5. `cargo clippy --features ads` — clean
- [x] 6. Update `docs/adsense.md` and `README.md` to point at `.env.sample`
      (3 references in README, 2 in adsense.md updated)
- [x] 7. Delete orphaned `.env.example` (now unreferenced; was the original
      footgun source). Useful content preserved in `.env.sample`.

## Notes

- `.env.sample` uses EMPTY value for `ADSENSE_CLIENT_ID` (not a placeholder
  string) to avoid the previous footgun where a copied placeholder leaked
  into the build.
- Other required env vars (`APP_MODE`, `SUPABASE_URL`, `SUPABASE_ANON_KEY`)
  are included with obvious placeholders so the template is actually usable
  end-to-end.
- `PLACEHOLDER_CLIENT_ID` env var is intentionally NOT exposed — it's an
  internal implementation detail that users shouldn't override.
- Historical `.plans/12-*.md` and `.plans/13-*.md` still reference the old
  `.env.example` filename — left untouched per project rule "Plan files
  (historical records — don't rewrite)".
- No changes to `build.rs` (it already supports `ADSENSE_CLIENT_ID` env var
  as priority #1).
- No rebuild/redeploy needed — `.env.sample` is documentation only; existing
  `.env` (if any) and `index.html` continue to work as before.
