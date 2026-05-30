use content_sdk::contexts::{ContentContext, ContentTagsContext, TagContext};
use content_sdk::models::Content as ContentModel;

use dioxus::prelude::*;

use crate::features::content::hooks::use_tags::use_tags;
use crate::features::content::inline_viewer::InlineContentViewer;
use crate::features::content::tag_filter_bar::TagFilterBar;

const EMPTY_TOPICS: &str = "Select a tag to view topics";
const EMPTY_CONTENT: &str = "Select a topic to read";

fn get_query_param(key: &str, query: &str) -> Option<String> {
    query.strip_prefix('?')?.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        if k == key { Some(v.to_string()) } else { None }
    })
}

fn replace_url(url: &str) {
    if let Some(window) = web_sys::window()
        && let Ok(h) = window.history()
    {
        let _ = h.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(url));
    }
}

#[component]
pub fn ContentPage() -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    let mut selected_tag = use_signal(|| None::<i32>);
    let mut tag_content = use_signal(Vec::<ContentModel>::new);
    let mut content_loading = use_signal(|| false);
    let mut selected_content = use_signal(|| None::<ContentModel>);
    let mut url_consumed = use_signal(|| false);

    let tag_ctx = use_context::<TagContext>();
    let content_ctx = use_context::<ContentContext>();
    let content_tags_ctx = use_context::<ContentTagsContext>();
    let ct_ctx_for_spawn = content_tags_ctx.clone();

    let tags_resource = use_tags(tag_ctx);
    let tags_list = use_memo(move || tags_resource().clone().unwrap_or_default());
    let tags_loaded = use_memo(move || tags_resource().is_some());

    let initial_query = use_context::<Signal<String>>();

    let content_ctx_for_effect = content_ctx.clone();
    let ct_ctx_for_effect = ct_ctx_for_spawn.clone();
    let mut initial_content_id: Signal<Option<i32>> = use_signal(|| None);

    use_effect(move || {
        if !url_consumed() && *tags_loaded.read() {
            url_consumed.set(true);
            if let Some(tag_id_str) = get_query_param("tag_id", &initial_query())
                && let Ok(tag_id) = tag_id_str.parse::<i32>()
                && selected_tag().is_none()
            {
                let cid = get_query_param("content_id", &initial_query())
                    .and_then(|s| s.parse::<i32>().ok());
                initial_content_id.set(cid);
                selected_tag.set(Some(tag_id));
                content_loading.set(true);
                let c_ctx = content_ctx_for_effect.clone();
                let ct_ctx = ct_ctx_for_effect.clone();
                spawn(async move {
                    let ids = ct_ctx
                        .get_content_ids_for_tag(tag_id)
                        .await
                        .unwrap_or_default();
                    let content = if ids.is_empty() {
                        vec![]
                    } else {
                        c_ctx.get_content_by_ids(&ids).await.unwrap_or_default()
                    };
                    tag_content.set(content);
                    content_loading.set(false);
                    if initial_content_id().is_none() {
                        replace_url(&format!("/content?tag_id={}", tag_id));
                    }
                });
            }
        }
    });

    use_effect(move || {
        if let Some(cid) = initial_content_id()
            && !content_loading()
        {
            initial_content_id.set(None);
            let found = tag_content().into_iter().find(|c| c.id == Some(cid));
            if let Some(content) = found {
                if let Some(tid) = selected_tag() {
                    replace_url(&format!("/content?tag_id={}&content_id={}", tid, cid,));
                }
                selected_content.set(Some(content));
            }
        }
    });

    rsx! {
        section {
            class: if dark_mode() { "content-page-layout" } else { "content-page-layout light-mode" },

            // Zone 1: Horizontal tag bar
            TagFilterBar {
                tags: tags_list(),
                selected_tag: selected_tag(),
                dark_mode,
                on_tag_select: move |tag_id: Option<i32>| {
                    selected_tag.set(tag_id);
                    selected_content.set(None);
                    if let Some(id) = tag_id {
                        replace_url(&format!("/content?tag_id={}", id));
                        content_loading.set(true);
                        let c_ctx = content_ctx.clone();
                        let ct_ctx = ct_ctx_for_spawn.clone();
                        spawn(async move {
                            let ids = ct_ctx
                                .get_content_ids_for_tag(id)
                                .await
                                .unwrap_or_default();
                            let content = if ids.is_empty() {
                                vec![]
                            } else {
                                c_ctx.get_content_by_ids(&ids).await.unwrap_or_default()
                            };
                            tag_content.set(content);
                            content_loading.set(false);
                        });
                    } else {
                        replace_url("/content");
                    }
                },
            }

            // Zone 2+3: Two-column layout
            div {
                class: "content-body-layout",

                // Left: Topics list
                div {
                    class: if dark_mode() { "content-topics-list" } else { "content-topics-list light-mode" },

                    if *content_loading.read() {
                        div { class: "loading", "Loading topics..." }
                    } else if selected_tag().is_none() {
                        div { class: "empty-state", "{EMPTY_TOPICS}" }
                    } else if tag_content().is_empty() {
                        div { class: "empty-state", "No topics for this tag" }
                    } else {
                        for item in tag_content() {
                            {
                                let is_active = selected_content()
                                    .as_ref()
                                    .map(|c| c.id == item.id)
                                    .unwrap_or(false);
                                let item_clone = item.clone();
                                let content_id = item.id;
                                let class_name = if is_active {
                                    "content-topics-item content-topics-item-active"
                                } else {
                                    "content-topics-item"
                                };
                                rsx! {
                                    button {
                                        class: "{class_name}",
                                        onclick: move |_| {
                                            selected_content.set(Some(item_clone.clone()));
                                            if let Some(cid) = content_id
                                                && let Some(tid) = selected_tag()
                                            {
                                                replace_url(&format!(
                                                    "/content?tag_id={}&content_id={}",
                                                    tid, cid,
                                                ));
                                            }
                                        },
                                        span { class: "content-topics-item-title", "{item.title}" }
                                        span { class: "content-topics-item-date",
                                            {item.created_at
                                                .map(|dt| dt.format("%Y-%m-%d").to_string())
                                                .unwrap_or_default()}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Right: Content viewer
                div {
                    class: "content-main",
                    if let Some(content) = selected_content() {
                        InlineContentViewer {
                            content: content.clone(),
                            tags_ctx: content_tags_ctx.clone(),
                        }
                    } else if !*content_loading.read() && selected_tag().is_some() {
                        div { class: "empty-state", "{EMPTY_CONTENT}" }
                    }
                }
            }
        }
    }
}
