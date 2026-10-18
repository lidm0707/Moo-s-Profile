# Content Loading Strategy

Current approach and the future per-tag plan for serving Supabase content on a
fully static site.

## Current: build-time snapshot embedded in WASM (in use)

- `make snapshot` runs `scripts/fetch-content-snapshot.mjs`, which pulls ALL
  published rows from Supabase (`content`, `tags`, `content_tags`) into
  `assets/content_snapshot.json`.
- `src/snapshot.rs` embeds the file via `include_str!` and serves every query
  (by slug, by ids, tags per content, ids per tag) from in-memory state.
- The site performs **zero** Supabase requests at runtime; the anon key is no
  longer shipped to the client.
- Data changes require `make snapshot` + rebuild (`make build` does both).

## Planned: per-tag JSON chunks (documented, NOT implemented yet)

Split the snapshot at build time so the browser only downloads what it opens:

```
dist/public/snap/
├── index.json        # small: all tags + slug → tag_id map (loaded on /content)
└── tag/
    ├── 1.json        # only the content items of tag 1 (+ their tags)
    ├── 2.json
    └── ...
```

Flow:

1. User opens `/content` → app fetches `snap/index.json` (a few KB) → renders
   the tag bar. Pages that never touch content (`/chat`, `/interests`) never
   download it.
2. User clicks a tag → app fetches `snap/tag/<id>.json` once, caches it in
   memory; switching back to a previously opened tag is instant.
3. Deep links (`/content/:slug`) resolve via the slug → tag_id map in
   `index.json`, then load that tag's chunk.

Trade-offs vs the current embedded snapshot:

| | embedded in WASM (now) | per-tag chunks (planned) |
|---|---|---|
| WASM size | grows with article count | fixed |
| First visit to /content | instant (no network) | 1 small request + 1 per new tag |
| Readers of only 1–2 tags | pay for everything | pay the least |
| Content in multiple tags | stored once | duplicated across chunk files (accepted) |
| Build script | one file | one file per tag |
| Runtime loading state | none | spinner only on first click of a tag |

Supabase is still only touched at build time; chunks are static files on the
CDN. Expected effort: extend `fetch-content-snapshot.mjs` to emit the chunk
tree, rewrite `src/snapshot.rs` from `OnceLock` + `include_str!` to async
fetch + in-memory cache, and return the affected call sites in
`content_grouped.rs` / `content_detail.rs` to async. UI and styles untouched.

Do this only when the embedded snapshot starts to bloat the WASM bundle.
