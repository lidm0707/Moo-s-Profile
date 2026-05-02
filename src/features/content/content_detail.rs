use chrono::Utc;
use content_sdk::contexts::{ContentContext, ContentTagsContext};
use content_sdk::models::Content as ContentModel;
use content_sdk::models::Tag;
use content_sdk::utils::render_markdown_to_html;
use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn ContentDetail(slug: String) -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    let content_ctx = use_context::<ContentContext>();
    let content_tags_ctx = use_context::<ContentTagsContext>();

    let content_resource = use_resource(move || {
        let ctx = content_ctx.clone();
        let slug_clone = slug.clone();
        async move { ctx.get_content_by_slug(&slug_clone).await }
    });

    let tags_resource = use_resource(move || {
        let ctx = content_tags_ctx.clone();
        let content_id = match content_resource.read().as_ref() {
            Some(Ok(Some(content))) => content.id,
            _ => None,
        };
        async move {
            if let Some(id) = content_id {
                ctx.get_tags_for_content(id).await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    });

    let is_loading = use_memo(move || content_resource.read().is_none());

    rsx! {
        section {
            class: if dark_mode() { "content-section" } else { "content-section light-mode" },

            div {
                class: "content-detail-header",
                Link {
                    to: Route::ContentPage {},
                    button {
                        class: if dark_mode() { "content-detail-back" } else { "content-detail-back light-mode" },
                        "← Back to Content"
                    }
                }
            }

            if *is_loading.read() {
                div {
                    class: if dark_mode() { "loading" } else { "loading light-mode" },
                    "Loading content..."
                }
            } else {
                {render_detail_state(&content_resource, &tags_resource, dark_mode)}
            }
        }
    }
}

fn render_detail_state(
    content_resource: &Resource<Result<Option<ContentModel>, String>>,
    tags_resource: &Resource<Vec<Tag>>,
    dark_mode: Signal<bool>,
) -> Element {
    match content_resource.read().as_ref() {
        Some(Err(e)) => rsx! {
            div {
                class: if dark_mode() { "error-state" } else { "error-state light-mode" },
                "Error loading content: {e}"
            }
        },
        Some(Ok(None)) => rsx! {
            div {
                class: if dark_mode() { "empty-state" } else { "empty-state light-mode" },
                "Content not found"
            }
        },
        Some(Ok(Some(content))) => {
            let title = content.title.clone();
            let date_str = content
                .created_at
                .map(|dt| dt.format("%B %e, %Y").to_string())
                .unwrap_or_else(|| Utc::now().format("%B %e, %Y").to_string());
            let body_html = render_markdown_to_html(&content.body);
            let tags = tags_resource().clone().unwrap_or_default();

            rsx! {
                article {
                    class: if dark_mode() { "content-detail" } else { "content-detail light-mode" },
                    header {
                        class: "content-detail-header",
                        h1 { class: "content-detail-title", "{title}" }
                    }
                    time {
                        class: "content-detail-date",
                        "{date_str}"
                    }
                    div {
                        class: "content-detail-body",
                        dangerous_inner_html: "{body_html}"
                    }
                    div {
                        class: "content-detail-tags",
                        for tag in tags {
                            span {
                                class: "content-tag",
                                "{tag.name}"
                            }
                        }
                    }
                }
            }
        }
        _ => rsx! { "" },
    }
}
