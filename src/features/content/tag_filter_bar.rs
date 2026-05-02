use content_sdk::models::Tag;
use dioxus::prelude::*;

#[component]
pub fn TagFilterBar(
    tags: Vec<Tag>,
    selected_tag: Option<i32>,
    dark_mode: Signal<bool>,
    on_tag_select: EventHandler<Option<i32>>,
) -> Element {
    rsx! {
        div {
            class: "tag-card-grid",
            for tag in tags.iter() {
                {
                    let tag_id = tag.id;
                    let is_active = selected_tag == tag_id;
                    let class_name = if is_active {
                        "tag-card tag-card-active"
                    } else if dark_mode() {
                        "tag-card"
                    } else {
                        "tag-card tag-card-light"
                    };
                    rsx! {
                        button {
                            class: "{class_name}",
                            onclick: move |_| on_tag_select.call(tag_id),
                            span { class: "tag-card-icon", "🏷️" }
                            span { class: "tag-card-name", "{tag.name}" }
                        }
                    }
                }
            }
        }
    }
}
