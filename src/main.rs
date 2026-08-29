mod components;
mod features;
mod hooks;
mod routes;
mod utils;

use dioxus::prelude::*;
use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const FONT_CSS: &str =
    "https://fonts.googleapis.com/css2?family=Zen+Maru+Gothic:wght@400;500;700;900&display=swap";
const MAIN_CSS: Asset = asset!(
    "/assets/main.css",
    CssAssetOptions::new().with_static_head(true)
);

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let initial_query: Signal<String> = use_signal(|| {
        web_sys::window()
            .and_then(|w| w.location().search().ok())
            .unwrap_or_default()
    });
    use_context_provider(|| initial_query);

    rsx! {
        document::Stylesheet { href: MAIN_CSS  }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "anonymous" }
        document::Link { rel: "stylesheet", href: FONT_CSS }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
