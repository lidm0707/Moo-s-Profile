mod components;
mod features;
mod hooks;
mod routes;

use dioxus::prelude::*;
use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
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
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
