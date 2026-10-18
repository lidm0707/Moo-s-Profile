use content_sdk::models::Content as ContentModel;
use content_sdk::utils::render_markdown_to_html;
use dioxus::prelude::*;

use crate::snapshot;

#[component]
pub fn ModalContentBody(content: ContentModel) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    let tags = content
        .id
        .map(snapshot::tags_for_content)
        .unwrap_or_default();

    let body_html = render_markdown_to_html(&content.body);

    rsx! {
        div {
            class: if dark_mode() { "modal-content-body" } else { "modal-content-body light-mode" },
            onclick: crate::utils::open_external_links_in_new_tab,
            dangerous_inner_html: "{body_html}"
        }
        div {
            class: "modal-content-tags",
            for tag in tags {
                span { class: "content-tag", "{tag.name}" }
            }
        }
    }
}
