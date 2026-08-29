use content_sdk::contexts::ContentTagsContext;
use content_sdk::models::Content as ContentModel;
use content_sdk::utils::render_markdown_to_html;
use dioxus::prelude::*;

use crate::components::Icon;
use crate::utils::copy_to_clipboard;

const SHARE_HINT: &str = "Copy link";
const COPIED_HINT: &str = "Copied!";

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

    let mut copied = use_signal(|| false);
    let share_url = move || {
        let origin = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_default();
        format!("{origin}/content/{}", content.slug)
    };
    let copy_share = move |_| {
        copy_to_clipboard(&share_url());
        copied.set(true);
        let window = web_sys::window();
        spawn(async move {
            // Reset the "Copied!" hint after a short delay.
            if let Some(window) = window {
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        resolve.as_ref(),
                        1_500,
                    );
                });
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            }
            copied.set(false);
        });
    };

    rsx! {
        article {
            class: if dark_mode() { "content-viewer" } else { "content-viewer light-mode" },
            header {
                class: "content-viewer-header",
                h2 { class: "content-viewer-title", "{content.title}" }
                div {
                    class: "content-viewer-meta",
                    time { class: "content-viewer-date", "{date_str}" }
                    button {
                        class: if copied() { "share-button copied" } else { "share-button" },
                        title: "Copy share link",
                        onclick: copy_share,
                        Icon { name: "share".to_string(), class: "share-icon" }
                        span {
                            if copied() { "{COPIED_HINT}" } else { "{SHARE_HINT}" }
                        }
                    }
                }
            }
            div {
                class: "content-viewer-body",
                onclick: crate::utils::open_external_links_in_new_tab,
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
