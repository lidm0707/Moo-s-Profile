# Plan 11: Realign Chat Page UI/UX With the Rest of the Site

**Created:** 2026-06-27
**Status:** Done

## Goal

The `/chat` page used its own royal-indigo/blue palette (`#0f0a2e`, `#1e1b4b`,
`#312e81`, `#4c1d95`) that didn't match the slate-gray + purple-accent design
language every other page shares (`#0f1116`/`#1f2937` base, `#a78bfa`/`#7c3aed`
accents, `#2a2d3a` dividers). Realign the chat page to that shared palette and
add the two highest-value chat UX touches.

## Tasks

- [x] 0. Realign chat palette in `assets/main.css` to the site's design language:
      - `.chat-app` dark bg → transparent (inherits `#0f1116` body); light → `#f8fafc`.
      - `.chat-header` / `.chat-input-bar` → `#1f2937` dark, `#f3f4f6` light; dividers → `#2a2d3a` / `#e5e7eb` (header uses the accent `#3730a3` / `#a78bfa` like `content-viewer-header`).
      - bot bubble → `#1f2937` bg + `#d1d5db` text + `#2a2d3a` border (matches `content-viewer-body`); light → `#ffffff` / `#374151` / `#e5e7eb`.
      - user bubble → brand accent `#7c3aed` (matches buttons / `pagination-active`).
      - avatar border → `#3730a3` / `#c4b5fd`; emoji avatar bg → `#1f2937` / `#ffffff`.
      - input → `#0f1116` bg, `#3730a3` border, focus `#a78bfa`; light `#ffffff` / `#c4b5fd` / focus `#7c3aed`.
      - status text → `#9ca3af` / `#6b7280` (matches muted text elsewhere).
- [x] 1. UX: auto-scroll the transcript to the newest message (`use_effect` on
      `messages.len()` + `scroll_chat_to_bottom` via `web_sys` `get_element_by_id`).
- [x] 2. UX: disable the Send button while the input is empty (derived via
      `use_memo`, no extra state) + `:disabled` / `:focus-visible` styling.
- [x] 3. Extract hardcoded strings to consts (`CHAT_LIST_ID`, `BOT_SEED_TEXT`, `INPUT_PLACEHOLDER`).
- [x] 4. Add `Document` + `Element` web-sys features for the scroll helper.
- [x] 5. Validate: `cargo check` && `cargo clippy`.

## Notes

- `window.document()` returns `Option<Document>` in this web-sys version (not `Result`).
- `Signal` is `Copy`, so reading `messages` inside `use_effect` subscribes the effect without extra plumbing.
- Did NOT add clickable example-prompt chips: LatCal's accepted syntax is only documented by one example, so unverified prompts risk showing error replies.
