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
            class: "tag-bar",
            for tag in tags.iter() {
                {
                    let tag_id = tag.id;
                    let is_active = selected_tag == tag_id;
                    let class_name = if is_active {
                        "tag-chip tag-chip-active"
                    } else {
                        "tag-chip"
                    };
                    let full_class = if dark_mode() {
                        class_name.to_string()
                    } else {
                        format!("{class_name} tag-chip-light")
                    };
                    rsx! {
                        button {
                            class: "{full_class}",
                            onclick: move |_| on_tag_select.call(tag_id),
                            "🏷 {tag.name}"
                        }
                    }
                }
            }
        }
    }
}
