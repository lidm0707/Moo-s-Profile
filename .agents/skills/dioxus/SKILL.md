---
name: dioxus
description: Helps with Dioxus 0.7 development in this project. Use this when writing components, managing signals, handling routing, using the content_sdk integration, or any Dioxus-specific work in Moo's Profile.
---

# Dioxus 0.7 — Moo's Profile Project

This project uses **Dioxus 0.7** with the `router` feature, targeting the `web` platform only. The API has changed significantly from earlier versions — `cx`, `Scope`, and `use_state` are gone.

## Project Setup

- **Build tool**: `dx serve` (Dioxus CLI)
- **Config**: `Dioxus.toml` at project root
- **Features**: `default = ["web"]`, `web = ["dioxus/web"]`
- **Key dependency**: `content_sdk` (git dependency from `https://github.com/lidm0707/content_profile`)

## Architecture

```
src/
├── main.rs          # App entry, launches Router::<Route>
├── components/      # Reusable UI components (header, footer, nav, modal, etc.)
├── features/        # Feature modules grouped by domain (content/)
├── hooks/           # Custom hooks (currently sparse)
└── routes/          # Route enum, layout component, page re-exports
```

### Module Convention

- Each module folder has a `mod.rs` that declares `pub mod` for sub-modules.
- `mod.rs` files do **not** `pub use` with `_` re-exports — they use explicit `pub use` with real names.
- Components are `pub use`'d from their parent `mod.rs` so routes can import them as `crate::components::Foo`.

## Route Pattern

Routes are defined in `src/routes/mod.rs` using a single `Route` enum with `#[layout(...)]`:

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(ProfileLayout)]
    #[route("/")]
    Home {},
    #[route("/interests")]
    Interests {},
    #[route("/work-history")]
    WorkHistory {},
    #[route("/content")]
    ContentPage {},
    #[route("/content/:slug")]
    ContentDetail { slug: String },
}
```

### Multiple Layouts (Important)

Dioxus 0.7 `#[layout(...)]` and `#[nest(...)]` always **nest** layouts — they never replace. If you add a second `#[layout(ContentLayout)]` under `#[nest("/content")]`, it renders **inside** `ProfileLayout`, giving you two nav bars stacked.

To have different layouts per route group, use `use_route()` in a single layout and conditionally render:

```rust
#[component]
pub fn ProfileLayout() -> Element {
    let route: Route = use_route();
    let is_content = matches!(route, Route::ContentPage {} | Route::ContentDetail { .. });

    if is_content {
        rsx! {
            Nav {}
            div { class: "content-layout", Outlet::<Route> {} }
        }
    } else {
        rsx! {
            ThemeToggle {}
            Header {}
            Nav {}
            main { class: "profile-content", Outlet::<Route> {} }
            Footer {}
        }
    }
}
```

### Route Details

- `ProfileLayout` is the shared layout (conditional rendering based on route).
- Content routes (`/content`, `/content/:slug`) render only Nav + full-height content.
- Other routes (`/interests`, `/work-history`) render Header + Nav + Footer.
- Dynamic segments (`:slug`) become struct fields on the enum variant.
- `Home` at `/` redirects to `/content` via `navigator().push()`.

## Component Conventions

- Annotate with `#[component]`.
- Function name starts with a capital letter.
- Props are owned values (`String`, `Vec<T>`, not `&str`).
- Props must implement `PartialEq` and `Clone`.
- Use `Signal<T>` for mutable reactive props; `ReadOnlySignal<T>` for read-only reactive props.
- Return type is `Element`.

### Component Example (from this project)

```rust
#[component]
pub fn Modal(is_open: Signal<bool>, title: String, children: Element) -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    if !is_open() {
        return rsx! {};
    }
    rsx! {
        div {
            class: if dark_mode() { "modal-backdrop" } else { "modal-backdrop light-mode" },
            onclick: move |_| is_open.set(false),
            div {
                class: "modal-container",
                onclick: move |e| e.stop_propagation(),
                {children}
            }
        }
    }
}
```

## State & Signals

- **`use_signal`** for local mutable state. Read with `signal()`, write with `signal.write()` or `signal.set(value)`.
- **`use_memo`** for derived values that recompute when dependencies change.
- **`use_context_provider`** / **`use_context`** for sharing state down the tree.
- Never use `use_state` or `cx` — those are pre-0.7 APIs.

### Context Pattern (from this project)

Dark mode is provided as `Signal<bool>` in `ProfileLayout`:

```rust
let dark_mode = use_signal(|| true);
use_context_provider(|| dark_mode);
// In any child:
let dark_mode = use_context::<Signal<bool>>();
```

## content_sdk Integration

The project uses `content_sdk` contexts initialized in `ProfileLayout`:

- `ContentContext` — fetches content entries
- `TagContext` — fetches tags
- `ContentTagsContext` — fetches content-tag associations

Each is initialized with a `Config` built from compile-time env vars (`env!()`):

```rust
let config = use_hook(|| {
    Config::new(
        env!("APP_MODE"),
        env!("SUPABASE_URL"),
        env!("SUPABASE_ANON_KEY"),
        None,
    )
});
use_context_provider(|| ContentContext::new(Some(config.clone())));
```

## RSX Patterns Used

- Conditional classes: `class: if dark_mode() { "dark" } else { "light" }`
- Iteration with `for` loops in RSX (preferred over `.map()`)
- `{children}` element passthrough for wrapper components
- Asset references via `asset!("/assets/foo.css")` with `CssAssetOptions`
- Early return `return rsx! {};` for conditional rendering

## URL Sharing (Query Params)

The content page uses `?tag_id=22` and `?tag_id=22&content_id=21` in the URL for sharing. The content_sdk uses integer IDs, not names. Since Dioxus 0.7 router doesn't support query params natively, we use `web-sys` + `wasm-bindgen`:

- **Read**: `web_sys::window().location().search()` → parse `?tag_id=22`
- **Write**: `history.replaceState()` via `web_sys` — updates URL without navigation
- **On mount**: `use_effect` reads `tag_id` and `content_id` from URL, selects matching tag/content
- **On tag click**: `replace_url("/content?tag_id=22")`
- **On topic click**: `replace_url("/content?tag_id=22&content_id=21")`

Dependencies: `web-sys = { version = "0.3", features = ["Window", "Location"] }` and `wasm-bindgen = "0.2"`.

## Styling

- CSS is loaded via `document::Stylesheet` in `App` or `ProfileLayout`.
- Dark/light mode toggled by appending `light-mode` class name.
- Static assets configured in `Dioxus.toml` under `[web.resource]`.

## Development Checklist

After making changes:

1. `cargo check` — verify compilation
2. `cargo clippy` — lint
3. Check `.plans/` for current task file and update status
4. No `#[allow(dead_code)]` — use `todo!()` or `unimplemented!()` instead
