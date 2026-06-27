# Plan 10: Add Chat Route with LatCal (latent-calculator)

**Created:** 2026-06-27
**Status:** Done (initial) — tweaked per follow-up

## Goal

Add a `/chat` route that uses the `latent-calculator` crate (LatCal) as a natural-language calculator. Users type NL math queries, results are derived directly from the input signal (no effect chains).

## Context

- LatCal repo: https://github.com/lidm0707/latent-calculator
- Readme says crate lives at `crates/latent-calculator` inside repo `https://github.com/lidm0707/katgpt-rs`, but the user-provided URL is `lidm0707/latent-calculator`. Use the user-specified git URL as the dependency source; Cargo resolves by package name.
- API: `Calculator::parse(&str) -> Result<Answer, ParseError>`; `Answer::to_sentence() -> String`.
- Project conventions: Dioxus 0.7, `Signal<T>` state, `use_memo` for derived, conditional dark-mode class, routes in `src/routes/mod.rs`, page components in `src/routes/<name>.rs`, nav links in `src/components/nav.rs`.

## Tasks

- [x] 0. Add `latent-calculator` git dependency to `Cargo.toml`.
- [x] 1. Create `src/routes/chat.rs` with the Chat page (single signal input + derived memo result).
- [x] 2. Register `Chat` route (`/chat`) in `src/routes/mod.rs` (enum variant + mod decl + re-export).
- [x] 3. Add a "Chat" nav link in `src/components/nav.rs`.
- [x] 4. Add minimal CSS for the chat page in `assets/main.css`.
- [x] 5. Validate: `cargo check` && `cargo clippy`.

## Tweak Round (2026-06-27)

Follow-up requests:
1. Chat page should use an isolated layout similar to content (Nav + fullscreen, no Header/Footer).
2. Chat must look like a real chat app — message bubbles, avatars (user = `profile.jpg` aka lidm, bot = emoji avatar), scrollable history, input bar at bottom.
3. Bot replies computed by LatCal (one bot bubble per user message).

### Tasks
- [x] 6. Route Chat through the standalone layout branch in `ProfileLayout`.
- [x] 7. Rewrite `src/routes/chat.rs` as a messenger-style UI (Signal<Vec<Message>>, send handler, bubbles + avatars).
- [x] 8. Add chat-app CSS in `assets/main.css`.
- [x] 9. Validate: `cargo check` && `cargo clippy`.

## Notes

- Keep Chat page in the non-content layout branch (Header + Nav + Footer) — it's a profile-page feature, not a content page. `is_content` match guard in `ProfileLayout` must NOT include `Chat`.
- Use the example from the LatCal readme as the base, adapted to this project's dark-mode + styling conventions.
- Derived state pattern: `use_memo` reads `input` signal and returns the sentence — no `use_effect`.
