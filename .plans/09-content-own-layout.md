# Plan 09: Content Page — Own Layout

**Created:** 2026-05-31
**Status:** Done

## Goal

Content page gets its own layout — just nav bar at top + full content area. No header, no footer, no profile info. Other pages (Work History) keep `ProfileLayout`.

## Problem

Current `Route` enum uses a single `#[layout(ProfileLayout)]` for ALL routes. Dioxus 0.7 supports multiple layout groups in one enum. Need to split into two layout groups.

## Layout

```
┌──────────────────────────────────────┐
│  Nav bar                             │
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

No header. No footer. Just nav + content.

## Tasks

### Task 1: Create `ContentLayout` component
**File:** `src/routes/mod.rs`

New layout component:
- Provides dark mode context
- Provides content_sdk contexts (ContentContext, TagContext, ContentTagsContext)
- Renders: Nav + full-height Outlet (no header, no footer, no theme toggle)

```
Status: [x] Done
```

### Task 2: Split Route enum into two layout groups
**File:** `src/routes/mod.rs`
```
Status: [x] Done
```

### Task 3: CSS for content-only layout
**File:** `assets/main.css`
```
Status: [x] Done
```

## Execution Order

```
All done.

## Validation

- `cargo check` — zero errors, zero warnings
- `cargo clippy` — clean
