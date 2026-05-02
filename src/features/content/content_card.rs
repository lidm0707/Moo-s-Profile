use content_sdk::contexts::ContentTagsContext;
use content_sdk::models::Content as ContentModel;
use dioxus::prelude::*;

#[component]
pub fn ContentCard(
    item: ContentModel,
    content_tags_ctx: ContentTagsContext,
    on_card_select: EventHandler<ContentModel>,
) -> Element {
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

    let item_for_click = item.clone();

    rsx! {
        article {
            class: if dark_mode() { "content-card content-card-clickable" } else { "content-card content-card-clickable light-mode" },
            onclick: move |_| on_card_select.call(item_for_click.clone()),

            header {
                class: "content-card-header",
                h3 { class: "content-card-title", "{item.title}" }
                time { class: "content-card-date", "{created_at}" }
            }
            div {
                class: "content-card-tags",
                if !*tags_loading.read() {
                    for tag in tags_list() {
                        span { class: "content-tag", "{tag.name}" }
                    }
                }
            }
            span {
                class: if dark_mode() { "content-card-hint" } else { "content-card-hint light-mode" },
                "Click to read →"
            }
        }
    }
}
