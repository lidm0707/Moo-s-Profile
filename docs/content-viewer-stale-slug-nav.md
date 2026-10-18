# Bug Fix: Viewer Not Updating When Switching Topics on a Slug Route

Date: 2026-10-18
Files: `src/features/content/content_grouped.rs`, `src/features/content/content_detail.rs`, `e2e/tests/content.spec.ts`

## Symptom

On the Content page: click a tag chip, open an article from the left topics
list (URL becomes `/content/:slug`), then click a different topic in the
sidebar. The URL changes to the new slug, but the article viewer in the
center keeps showing the old article. Only a full page reload fixes it.

## Root Cause

The content browser (`ContentPage`) is stateful: `selected_tag`,
`tag_content`, `selected_content` signals, plus a deep-link `use_effect`
that consumes a `?tag_id=…&content_id=…` query to seed that state.

Two facts combined into the bug:

1. **slug → slug navigation reuses the mounted component.** Going from
   `/content/quick-sort` to `/content/recursive` is the same route pattern,
   so the Dioxus router keeps `ContentDetail`/`ContentPage` mounted and only
   updates props. Nothing remounts, so all signals keep their old values.
2. **`deep_link` was a plain prop (`Option<String>`), not a signal.**
   Reading a plain prop inside `use_effect` creates no dependency. The
   effect ran once on mount; when the prop changed on later navigation the
   effect never re-ran, so `selected_content` was never updated.

Dead ends ruled out along the way (kept here so nobody retries them):

- A one-shot `link_consumed` guard: irrelevant, the effect never ran again
  at all.
- `key: "{content.id}"` on `<ContentPage>` to force a remount: the keyed
  remount did not happen on prop-only re-renders. Proven with a temporary
  debug element — the component re-rendered with the new slug while its
  internal state stayed stale.

## Fix

Make the deep-link prop reactive so the effect can observe it:

```rust
// content_grouped.rs
pub fn ContentPage(
    #[props(default)] deep_link: ReadSignal<Option<String>>,
) -> Element {
```

- The effect now reads `deep_link()`, which tracks the signal — every prop
  change re-runs the effect.
- The old one-shot `link_consumed` guard was replaced with
  `last_consumed: Signal<Option<String>>`, which skips only when the link
  is unchanged (avoids redundant work on unrelated re-renders).
- `ReadSignal` is used instead of the deprecated `ReadOnlySignal`
  (removed in Dioxus 0.8).
- `ContentDetail` still passes a plain value; the component macro wraps it.

## Regression Test

`e2e/tests/content.spec.ts` — `switching topics from the sidebar updates
the viewer on a slug route`:

- Helpers `publishedSlugs()` / `firstTagChipWithTwoTopics()` read
  `assets/content_snapshot.json` (same snapshot embedded in the build) to
  pick a tag chip with ≥ 2 published articles.
- Replays the user scenario: click tag → open first topic → read its
  viewer title → click the second topic → assert the viewer title changed
  and the URL is a slug URL.

## Validation

- `make test` — cargo check + clippy `-D warnings`: pass.
- `make build` (fresh `dist/public`) then `make test-e2e` (docker):
  **8 passed, 0 failed** (7 existing + 1 new).

## Operational Note

`make build` deletes and recreates `dist/public`. If the `web` nginx
container is already running, its bind mount points at the deleted inode
and serves `500`s until recreated:

```sh
docker compose -f docker/docker-compose.yml up -d --force-recreate web
```

`make test-e2e` is unaffected (`up --build` recreates services).
