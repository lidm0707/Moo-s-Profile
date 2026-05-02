# Plan 06: Add Modal Component

**Created:** 2026-05-02
**Status:** Done

---

## Goal

Create a reusable `Modal` component at `src/components/modal.rs` that can be used throughout the app.

### Component Design

```
Modal
├── Props: is_open (Signal<bool>), title (String), children (Element)
├── Backdrop click → closes modal
├── Escape key → closes modal
├── Dark/light mode support
└── Reusable for: content preview, confirmations, forms, etc.
```

### Props

| Prop | Type | Description |
|------|------|-------------|
| `is_open` | `Signal<bool>` | Reactive open/close state |
| `title` | `String` | Modal header title |
| `children` | `Element` | Modal body content |

### File Changes

| # | File | Action |
|---|------|--------|
| 1 | `src/components/modal.rs` | Create — Modal component |
| 2 | `src/components/mod.rs` | Update — add `pub mod modal` + re-export |
| 3 | `assets/main.css` | Update — add `.modal*` styles |

---

## Tasks

### Phase 1 — Create Modal component
- [x] Create `src/components/modal.rs`
  - `Modal` component with `is_open`, `title`, `children` props
  - Backdrop click closes
  - Dark/light mode
- [x] Update `src/components/mod.rs` — add `pub mod modal` + re-export
- [x] Add `.modal*` CSS styles to `assets/main.css`

### Phase 2 — Verify
- [x] `dx check` passes
- [x] `cargo clippy` — 1 warning (unused `pub use Modal` — resolves when component is consumed)

---

## Constraints

- No `match` in `rsx!` — extract to functions
- Props: owned values, `PartialEq + Clone`
- RSX blocks under 30 lines
- No `#[allow(dead_code)]`
- No hardcoding — use const
- Dark/light mode support
