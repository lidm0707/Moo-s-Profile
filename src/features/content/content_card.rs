use content_sdk::contexts::ContentTagsContext;
use content_sdk::models::Content as ContentModel;
use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn ContentCard(item: ContentModel, content_tags_ctx: ContentTagsContext) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    let tags_resource = use_resource(move || {
        let ctx = content_tags_ctx.clone();
        let content_id = item.id;
        async move {
            if let Some(id) = content_id {
                ctx.get_tags_for_content(id).await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    });

    let tags_list = use_memo(move || tags_resource().clone().unwrap_or_default());
    let tags_loading = use_memo(move || tags_resource().is_none());

    let created_at = item
        .created_at
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    rsx! {
        article {
            class: if dark_mode() { "content-card" } else { "content-card light-mode" },
            header {
                class: "content-card-header",
                h3 { class: "content-card-title", "{item.title}" }
                time { class: "content-card-date", "{created_at}" }
            }
            div {
                class: "content-card-tags",
                if !*tags_loading.read() {
                    for tag in tags_list() {
                        span {
                            class: "content-tag",
                            "{tag.name}"
                        }
                    }
                }
            }
            Link {
                to: Route::ContentDetail { slug: item.slug.clone() },
                button {
                    class: if dark_mode() { "content-card-button" } else { "content-card-button light-mode" },
                    "Read More →"
                }
            }
        }
    }
}
