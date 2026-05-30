use content_sdk::contexts::ContentTagsContext;
use content_sdk::models::Content as ContentModel;
use content_sdk::utils::render_markdown_to_html;
use dioxus::prelude::*;

#[component]
pub fn InlineContentViewer(content: ContentModel, tags_ctx: ContentTagsContext) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    let tags_resource = use_resource(move || {
        let ctx = tags_ctx.clone();
        let content_id = content.id;
        async move {
            if let Some(id) = content_id {
                ctx.get_tags_for_content(id).await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    });

    let tags = use_memo(move || tags_resource().clone().unwrap_or_default());

    let date_str = content
        .created_at
        .map(|dt| dt.format("%B %e, %Y").to_string())
        .unwrap_or_default();

    let body_html = render_markdown_to_html(&content.body);

    rsx! {
        article {
            class: if dark_mode() { "content-viewer" } else { "content-viewer light-mode" },
            header {
                class: "content-viewer-header",
                h2 { class: "content-viewer-title", "{content.title}" }
                time { class: "content-viewer-date", "{date_str}" }
            }
            div {
                class: "content-viewer-body",
                dangerous_inner_html: "{body_html}"
            }
            div {
                class: "content-viewer-tags",
                for tag in tags() {
                    span { class: "content-tag", "{tag.name}" }
                }
            }
        }
    }
}
