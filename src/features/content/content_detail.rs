use dioxus::prelude::*;

use crate::features::content::content_grouped::ContentPage;
use crate::snapshot;

const NOT_FOUND: &str = "Content not found";

/// Entry point for `/content/:slug`.
///
/// The slug URL is the canonical, SEO-friendly link (the one listed in
/// `sitemap.xml`). It renders the full browser UI directly at the slug URL —
/// no route replace — so crawlers and users stay on the canonical address.
/// The first tag of the content is pre-selected to populate the topics list.
#[component]
pub fn ContentDetail(slug: String) -> Element {
    let Some(content) = snapshot::content_by_slug(&slug) else {
        // Snapshot streams in at boot; don't flash "not found" before it lands.
        let msg = if snapshot::loaded() {
            NOT_FOUND
        } else {
            snapshot::LOADING
        };
        return rsx! {
            section {
                class: "content-page-layout",
                div { class: "empty-state", "{msg}" }
            }
        };
    };

    rsx! {
        ContentPage { deep_link: build_deep_link_query(content.id) }
    }
}

/// Builds the `?tag_id=…&content_id=…` query consumed by `ContentPage`'s
/// deep-link effect. Uses the content's first tag if any; otherwise just the
/// content id (the inline viewer still renders, the topics list just stays
/// empty).
fn build_deep_link_query(content_id: Option<i32>) -> String {
    let Some(content_id) = content_id else {
        return String::new();
    };
    let content_id_part = format!("content_id={}", content_id);
    match snapshot::tags_for_content(content_id)
        .first()
        .and_then(|t| t.id)
    {
        Some(tag_id) => format!("?tag_id={}&{}", tag_id, content_id_part),
        None => format!("?{}", content_id_part),
    }
}
