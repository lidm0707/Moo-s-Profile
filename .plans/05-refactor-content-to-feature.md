# Plan 05: Refactor Content Route → Feature Module

**Created:** 2026-05-02
**Status:** Done

---

## Goal

Move `src/routes/content.rs` (monolith ~300 lines) into `src/features/content/` as small, focused modules.

### Split Strategy

```
src/features/content/
├── mod.rs                    # pub mod re-exports
├── tag_content_map.rs        # TagContentMap struct + mapping logic
├── tag_filter_bar.rs         # TagFilterBar component
├── content_card.rs           # ContentCard component
├── content_grouped.rs        # ContentPage (listing + tag groups)
├── content_detail.rs         # ContentDetail + render_detail_state
└── hooks/
    ├── mod.rs                # pub mod re-exports
    ├── use_tags.rs           # use_resource for tags
    ├── use_content.rs        # use_resource for content
    └── use_tag_mapping.rs    # use_resource for tag↔content mapping
```

### Module Responsibility

| # | Module | Responsibility |
|---|--------|----------------|
| 1 | `tag_content_map.rs` | `TagContentMap` struct, no logic change |
| 2 | `tag_filter_bar.rs` | `TagFilterBar` component — render tag buttons, filter by selected |
| 3 | `content_grouped.rs` | `ContentPage` — compose hooks, `grouped` memo, `content_without_tags` memo, render groups |
| 4 | `content_detail.rs` | `ContentDetail` + `render_detail_state` — open single content by slug |
| 5 | `hooks/use_tags.rs` | `use_tags()` → `Resource<Vec<Tag>>` |
| 6 | `hooks/use_content.rs` | `use_all_content()` → `Resource<Vec<ContentModel>>` |
| 7 | `hooks/use_tag_mapping.rs` | `use_tag_mapping(tags)` → `Resource<TagContentMap>` |

### First-load UX

- On first load, tags are fetched and displayed in `TagFilterBar`.
- User **must** select a tag before grouped content appears.
- `selected_tag` starts as `None`; show tag buttons + hint text "Select a tag to view content".

---

## Tasks

### Phase 1 — Create directory + stubs
- [x] `mkdir -p src/features/content/hooks`
- [x] Create `tag_content_map.rs` — move `TagContentMap` struct
- [x] Create `tag_filter_bar.rs` — move `TagFilterBar` component
- [x] Create `content_card.rs` — move `ContentCard` component
- [x] Create `content_grouped.rs` — move `ContentPage` with updated imports
- [x] Create `content_detail.rs` — move `ContentDetail` + `render_detail_state`
- [x] Create `hooks/use_tags.rs`
- [x] Create `hooks/use_content.rs`
- [x] Create `hooks/use_tag_mapping.rs`
- [x] Create `hooks/mod.rs`
- [x] Create `features/content/mod.rs`

### Phase 2 — Wire up
- [x] Update `src/routes/content.rs` → re-export from `crate::features::content`
- [x] Verify `Route` in `routes/mod.rs` still resolves `ContentPage` & `ContentDetail`
- [x] `dx check` passes

### Phase 3 — UX tweak: force tag selection first
- [x] In `ContentPage`, when `selected_tag` is `None`:
  - Show only `TagFilterBar` + hint text
  - Do NOT render content groups
- [x] When tag is selected, render grouped content

### Phase 4 — Cleanup & verify
- [x] Remove dead code from `src/routes/content.rs` (if any)
- [x] `dx check` — no issues found
- [x] `cargo clippy` — zero warnings
- [x] Update this plan status to Done

---

## Constraints

- No `match` in `rsx!` — extract to functions (already done for `render_detail_state`)
- All async fetching via `use_resource` — hooks wrap these
- Props: owned values, `PartialEq + Clone`
- RSX blocks under 30 lines each
- No `#[allow(dead_code)]` — use `todo!()` if needed
- No hardcoding — use `const`
