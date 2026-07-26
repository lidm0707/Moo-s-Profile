use content_sdk::contexts::{ContentContext, ContentTagsContext};
use dioxus::prelude::*;

use crate::routes::Route;

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
    let dark_mode = use_context::<Signal<bool>>();
    let mut initial_query = use_context::<Signal<String>>();
    let nav = navigator();

    let content_ctx = use_context::<ContentContext>();
    let content_tags_ctx = use_context::<ContentTagsContext>();

    let mut resolved = use_signal(|| false);

    let content_resource = use_resource(move || {
        let ctx = content_ctx.clone();
        let slug_clone = slug.clone();
        async move { ctx.get_content_by_slug(&slug_clone).await }
    });

    let ct_ctx = content_tags_ctx.clone();
    let nav_clone = nav;

    use_effect(move || {
        if resolved() {
            return;
        }
        let resource_ref = content_resource.read();
        let Some(result) = resource_ref.as_ref() else {
            return;
        };
        let content_opt = match result {
            Ok(c) => c.clone(),
            Err(_) => {
                resolved.set(true);
                return;
            }
        };
        let Some(content) = content_opt else {
            resolved.set(true);
            return;
        };
        let Some(content_id) = content.id else {
            resolved.set(true);
            return;
        };

        resolved.set(true);

        let ct_ctx = ct_ctx.clone();
        spawn(async move {
            let query = build_deep_link_query(&ct_ctx, content_id).await;
            initial_query.set(query);
            nav_clone.replace(Route::ContentPage {});
        });
    });

    rsx! {
        section {
            class: if dark_mode() { "content-page-layout" } else { "content-page-layout light-mode" },
            div { class: "loading", "{LOADING_TEXT}" }
        }
    }
}

/// Builds the `?tag_id=…&content_id=…` query for `ContentPage`'s deep-link
/// effect. Uses the content's first tag if any; otherwise just the content id
/// (the inline viewer still renders, the topics list just stays empty).
async fn build_deep_link_query(ct_ctx: &ContentTagsContext, content_id: i32) -> String {
    let content_id_part = format!("content_id={}", content_id);
    match ct_ctx.get_tags_for_content(content_id).await {
        Ok(tags) => match tags.first().and_then(|t| t.id) {
            Some(tag_id) => format!("?tag_id={}&{}", tag_id, content_id_part),
            None => format!("?{}", content_id_part),
        },
        Err(_) => format!("?{}", content_id_part),
    }
}
