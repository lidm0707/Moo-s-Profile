use dioxus::prelude::*;

use crate::routes::Route;
use crate::snapshot;

const LOADING_TEXT: &str = "Loading content...";

/// Entry point for `/content/:slug`.
///
/// The slug URL is the canonical, SEO-friendly link (the one listed in
/// `sitemap.xml`). It does not render its own layout — it resolves the slug to
/// a content id + first tag and hands control to `ContentPage` via the shared
/// `initial_query` signal + a route replace. Result: the slug route looks
/// identical to browsing from the index (tag bar + topics list + viewer).
#[component]
pub fn ContentDetail(slug: String) -> Element {
    let mut initial_query = use_context::<Signal<String>>();
    let nav = navigator();

    let mut resolved = use_signal(|| false);

    use_effect(move || {
        if resolved() {
            return;
        }
        resolved.set(true);

        let Some(content) = snapshot::content_by_slug(&slug) else {
            return;
        };
        let Some(content_id) = content.id else {
            return;
        };

        initial_query.set(build_deep_link_query(content_id));
        nav.replace(Route::ContentPage {});
    });

    rsx! {
        section {
            class: "content-page-layout",
            div { class: "loading", "{LOADING_TEXT}" }
        }
    }
}

/// Builds the `?tag_id=…&content_id=…` query for `ContentPage`'s deep-link
/// effect. Uses the content's first tag if any; otherwise just the content id
/// (the inline viewer still renders, the topics list just stays empty).
fn build_deep_link_query(content_id: i32) -> String {
    let content_id_part = format!("content_id={}", content_id);
    match snapshot::tags_for_content(content_id)
        .first()
        .and_then(|t| t.id)
    {
        Some(tag_id) => format!("?tag_id={}&{}", tag_id, content_id_part),
        None => format!("?{}", content_id_part),
    }
}
