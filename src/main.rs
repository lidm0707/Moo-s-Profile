mod components;
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
    rsx! {
        document::Stylesheet { href: MAIN_CSS  }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
