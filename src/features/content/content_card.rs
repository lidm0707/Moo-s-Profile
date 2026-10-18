use content_sdk::models::Content as ContentModel;
use dioxus::prelude::*;

use crate::snapshot;

#[component]
pub fn ContentCard(item: ContentModel, on_card_select: EventHandler<ContentModel>) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    let tags_list = item.id.map(snapshot::tags_for_content).unwrap_or_default();

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
                for tag in tags_list {
                    span { class: "content-tag", "{tag.name}" }
                }
            }
            span {
                class: if dark_mode() { "content-card-hint" } else { "content-card-hint light-mode" },
                "Click to read →"
            }
        }
    }
}
