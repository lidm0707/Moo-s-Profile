# Plan 07: Content Card → Modal Preview

**Created:** 2026-05-02
**Status:** Done

---

## Goal

Click a `ContentCard` → open `Modal` with full content rendered (markdown body + tags) instead of navigating to `ContentDetail` route.

### Current Flow

```
ContentCard → "Read More →" button → Link to Route::ContentDetail { slug }
ContentDetail fetches: content by slug + tags by content_id
```

### New Flow

```
ContentCard → click whole card → open Modal with content body + tags
No route change. ContentModel already available from card props.
```

### Key Insight

`ContentModel` from the card already has `body` (markdown) and `created_at`. Tags are already fetched per-card via `content_tags_ctx`. We can render everything without extra API calls.

### Architecture

```
content_grouped.rs (ContentPage)
├── owns: modal_open Signal<bool>
├── owns: selected_content Signal<Option<ContentModel>>
├── passes on_card_select EventHandler to ContentCard
│
├── Modal { is_open, title }
│   └── ModalContentBody { content, tags }
│       ├── date
│       ├── rendered markdown body
│       └── tag pills
```

### File Changes

| # | File | Action |
|---|------|--------|
| 1 | `src/features/content/content_card.rs` | Modify — replace Link with `on_card_select` callback |
| 2 | `src/features/content/content_grouped.rs` | Modify — add modal state, wire `on_card_select` |
| 3 | `src/features/content/mod.rs` | Modify — add `pub mod modal_content` |
| 4 | `src/features/content/modal_content.rs` | Create — modal body rendering (markdown + tags + date) |
| 5 | `assets/main.css` | Update — add `.modal-content*` styles |

---

## Tasks

### Phase 1 — Create modal content renderer
- [x] Create `src/features/content/modal_content.rs`
  - `ModalContentBody` component
  - Render: date, markdown body (via `render_markdown_to_html`), tag pills
  - Dark/light mode
- [x] Update `src/features/content/mod.rs` — add `pub mod modal_content`

### Phase 2 — Wire card → modal
- [x] Update `content_card.rs` — replace `Link` button with `on_card_select` EventHandler on whole card
- [x] Update `content_grouped.rs` — add `modal_open` + `selected_content` signals, render `Modal`

### Phase 3 — CSS
- [x] Add modal content styles to `assets/main.css`

### Phase 4 — Verify
- [x] `dx check` passes
- [x] `cargo clippy` — zero warnings
- [x] `cargo test` passes

---

## Constraints

- No `match` in `rsx!` — extract to functions
- Props: owned values, `PartialEq + Clone`
- RSX blocks under 30 lines
- No `#[allow(dead_code)]`
- No hardcoding
- Dark/light mode support
- Lean: reuse existing `Modal` component
