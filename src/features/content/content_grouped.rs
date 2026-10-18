use content_sdk::models::Content as ContentModel;

use dioxus::prelude::*;

use crate::components::Adsense;
use crate::features::content::inline_viewer::InlineContentViewer;
use crate::features::content::tag_filter_bar::TagFilterBar;
use crate::routes::Route;
use crate::snapshot;

const EMPTY_TOPICS: &str = "No topics for this tag";
const EMPTY_CONTENT: &str = "Select a topic to read";
const CONTENT_PAGE_AD_SLOT: &str = "7363546319";

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

fn content_for_tag(tag_id: i32) -> Vec<ContentModel> {
    let ids = snapshot::content_ids_for_tag(tag_id);
    snapshot::content_by_ids(&ids)
}

fn format_date<Tz: chrono::TimeZone>(created_at: Option<chrono::DateTime<Tz>>) -> String
where
    Tz::Offset: std::fmt::Display,
{
    created_at
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

#[component]
pub fn ContentPage(
    // ReadOnlySignal so the deep-link effect re-runs when slug→slug navigation
    // reuses this mounted component with a new link.
    #[props(default)] deep_link: ReadSignal<Option<String>>,
) -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    let mut selected_tag = use_signal(|| None::<i32>);
    let mut tag_content = use_signal(Vec::<ContentModel>::new);
    let mut selected_content = use_signal(|| None::<ContentModel>);
    // Track the last consumed deep link so a *changed* link (e.g. navigating
    // between slugs, which reuses the mounted component) is applied again.
    let mut last_consumed = use_signal(|| Option::<String>::None);

    let tags_list = snapshot::all_tags();

    let sorted_content = use_memo(move || {
        let mut items = tag_content();
        items.sort_by_key(|item| std::cmp::Reverse(item.created_at));
        items
    });

    let mut initial_query = use_context::<Signal<String>>();
    let route = use_route::<Route>();
    // On a slug URL the address bar is already canonical; never rewrite it to
    // a query-param URL.
    let on_slug_route = matches!(route, Route::ContentDetail { .. });
    let nav = navigator();

    use_effect(move || {
        // Deep links resolve against the snapshot; wait for it before consuming.
        // Reading loaded() here re-runs the effect when the snapshot arrives.
        if !snapshot::loaded() {
            return;
        }
        let query = match deep_link() {
            Some(link) => {
                // Don't let a stale deep link leak into later route mounts.
                initial_query.set(String::new());
                Some(link)
            }
            None => {
                let q = initial_query();
                (!q.is_empty()).then_some(q)
            }
        };
        let Some(query) = query else {
            return;
        };
        if last_consumed().as_deref() == Some(query.as_str()) {
            return;
        }
        last_consumed.set(Some(query.clone()));
        let Some(tag_id_str) = get_query_param("tag_id", &query) else {
            return;
        };
        if let Ok(tag_id) = tag_id_str.parse::<i32>() {
            selected_tag.set(Some(tag_id));
            tag_content.set(content_for_tag(tag_id));
            if let Some(cid) =
                get_query_param("content_id", &query).and_then(|s| s.parse::<i32>().ok())
            {
                selected_content.set(tag_content().into_iter().find(|c| c.id == Some(cid)));
            }
        }
    });

    // Default view: every published article, newest first, as crawlable links.
    let default_topics = use_memo(move || {
        let mut items = snapshot::all_published();
        items.sort_by_key(|item| std::cmp::Reverse(item.created_at));
        items
    });

    rsx! {
        section {
            class: if dark_mode() { "content-page-layout" } else { "content-page-layout light-mode" },

            // Zone 1: Horizontal tag bar
            TagFilterBar {
                tags: tags_list,
                selected_tag: selected_tag(),
                dark_mode,
                on_tag_select: move |tag_id: Option<i32>| {
                    selected_tag.set(tag_id);
                    selected_content.set(None);
                    if on_slug_route {
                        // Leave the canonical slug URL: go back to the index
                        // with the chosen tag pre-selected.
                        if let Some(id) = tag_id {
                            initial_query.set(format!("?tag_id={}", id));
                        }
                        nav.push(Route::ContentPage {});
                        return;
                    }
                    if let Some(id) = tag_id {
                        replace_url(&format!("/content?tag_id={}", id));
                        tag_content.set(content_for_tag(id));
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

                    if selected_tag().is_none() {
                        if default_topics().is_empty() && !snapshot::loaded() {
                            div { class: "empty-state", "{snapshot::LOADING}" }
                        } else {
                            for item in default_topics() {
                                TopicLink { item: item.clone(), active_route: route.clone() }
                            }
                        }
                    } else if !snapshot::loaded() {
                        div { class: "empty-state", "{snapshot::LOADING}" }
                    } else if tag_content().is_empty() {
                        div { class: "empty-state", "{EMPTY_TOPICS}" }
                    } else {
                        for item in sorted_content() {
                            TopicLink { item: item.clone(), active_route: route.clone() }
                        }
                    }
                }

                // Right: Content viewer
                div {
                    class: "content-main",
                    if let Some(content) = selected_content() {
                        InlineContentViewer { content: content.clone() }
                    } else {
                        div { class: "empty-state", "{EMPTY_CONTENT}" }
                    }
                }
            }

            // Live AdSense slot (lilidm article ad unit). Renders nothing when
            // the `ads` feature is off or the publisher ID is the placeholder.
            Adsense {
                ad_slot: CONTENT_PAGE_AD_SLOT.to_string(),
                dark_mode: Some(dark_mode()),
            }
        }
    }
}

#[component]
fn TopicLink(item: ContentModel, active_route: Route) -> Element {
    let is_active = matches!(&active_route, Route::ContentDetail { slug } if *slug == item.slug);
    let class_name = if is_active {
        "content-topics-item content-topics-item-active"
    } else {
        "content-topics-item"
    };
    let date_str = format_date(item.created_at);

    rsx! {
        Link {
            class: "{class_name}",
            to: Route::ContentDetail { slug: item.slug.clone() },
            span { class: "content-topics-item-title", "{item.title}" }
            span { class: "content-topics-item-date", "{date_str}" }
        }
    }
}
