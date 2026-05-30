# Plan 08: Content Page Redesign — Tag Bar + Two-Column Layout

**Created:** 2026-05-30
**Status:** Done

## Goal

Redesign the content page as the landing page with a three-zone layout: horizontal tag bar at top, topics list on left, content on right. Other pages unchanged. Mobile responsive.

## Layout

```
┌──────────────────────────────────────┐
│  Header / Nav                        │
├──────────────────────────────────────┤
│  Tags (horizontal bar, small chips)  │
│  [🏷 Rust] [🏷 Crypto] [🏷 ESP32]    │
├────────────┬─────────────────────────┤
│  Topics    │  Content                │
│  (left)    │  (right)                │
│  - Topic1  │  InlineContentViewer    │
│  - Topic2  │                         │
│  - Topic3  │                         │
└────────────┴─────────────────────────┘
```

**Mobile:**
- Tag bar scrolls horizontally
- Topics list stacks above content (max-height 250px, scrollable)
- Content viewer fills remaining space below

**Other pages** (Work History) stay unchanged.

## Tweaks Applied

1. **Content page fills full area** — no extra title, just the 3-zone layout. Other pages still use `profile-content` padding.
2. **Tags are small chips** — inline icon + text (e.g. `🏷 Rust`), pill-shaped, not large cards.
3. **Mobile responsive** — tag bar scrolls horizontally, topics/content stack vertically.

## Tasks

| # | Task | Status |
|---|------|--------|
| 1 | Redirect `/` → `/content` | Done |
| 2 | Remove Interests from nav | Done |
| 3 | Redesign ContentPage — 3-zone layout | Done |
| 4 | Inline content viewer component | Done |
| 5 | Tag in URL via `?tag=name` | Deferred |
| 6 | CSS for new layout + mobile | Done |
| 7 | Clean up unused imports | Done |

## Validation

- `cargo check` — zero errors, zero warnings
- `cargo clippy` — clean

## Files Changed

| File | Change |
|------|--------|
| `src/routes/mod.rs` | Home redirect, removed Interests route |
| `src/components/nav.rs` | Removed Interests nav link |
| `src/components/mod.rs` | Removed unused pub use |
| `src/features/content/content_grouped.rs` | 3-zone layout rewrite |
| `src/features/content/inline_viewer.rs` | New inline content viewer |
| `src/features/content/tag_filter_bar.rs` | Small chip style (🏷 + text) |
| `src/features/content/mod.rs` | Added inline_viewer module |
| `assets/main.css` | New layout CSS + mobile responsive |
