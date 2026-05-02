use content_sdk::contexts::{ContentContext, ContentTagsContext, TagContext};
use content_sdk::models::Content as ContentModel;

use dioxus::prelude::*;

use crate::components::Modal;
use crate::features::content::content_card::ContentCard;
use crate::features::content::hooks::use_tags::use_tags;
use crate::features::content::modal_content::ModalContentBody;
use crate::features::content::tag_filter_bar::TagFilterBar;

const HINT_TEXT: &str = "Select a tag to view content";

#[component]
pub fn ContentPage() -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    let mut selected_tag = use_signal(|| None::<i32>);
    let mut tag_content = use_signal(Vec::<ContentModel>::new);
    let mut content_loading = use_signal(|| false);
    let mut modal_open = use_signal(|| false);
    let mut selected_content = use_signal(|| None::<ContentModel>);

    let tag_ctx = use_context::<TagContext>();
    let content_ctx = use_context::<ContentContext>();
    let content_tags_ctx = use_context::<ContentTagsContext>();
    let ct_ctx_for_spawn = content_tags_ctx.clone();

    let tags_resource = use_tags(tag_ctx);

    let tags_loading = use_memo(move || tags_resource().is_none());
    let tags_list = use_memo(move || tags_resource().clone().unwrap_or_default());

    let active_tag_name = use_memo(move || {
        let Some(active_id) = selected_tag() else {
            return String::new();
        };
        tags_list()
            .iter()
            .find(|t| t.id == Some(active_id))
            .map(|t| t.name.clone())
            .unwrap_or_default()
    });

    let modal_title = use_memo(move || {
        selected_content()
            .as_ref()
            .map(|c| c.title.clone())
            .unwrap_or_default()
    });

    rsx! {
        section {
            class: if dark_mode() { "content-section" } else { "content-section light-mode" },
            h2 { "📚 Content" }

            if *tags_loading.read() {
                div {
                    class: if dark_mode() { "loading" } else { "loading light-mode" },
                    "Loading tags..."
                }
            } else {
                TagFilterBar {
                    tags: tags_list(),
                    selected_tag: selected_tag(),
                    dark_mode,
                    on_tag_select: move |tag_id: Option<i32>| {
                        selected_tag.set(tag_id);
                        if let Some(id) = tag_id {
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
                                    c_ctx
                                        .get_content_by_ids(&ids)
                                        .await
                                        .unwrap_or_default()
                                };

                                tag_content.set(content);
                                content_loading.set(false);
                            });
                        }
                    },
                }

                if *content_loading.read() {
                    div {
                        class: if dark_mode() { "loading" } else { "loading light-mode" },
                        "Loading content..."
                    }
                } else if selected_tag().is_none() {
                    div {
                        class: if dark_mode() { "empty-state" } else { "empty-state light-mode" },
                        "{HINT_TEXT}"
                    }
                } else {
                    div {
                        class: if dark_mode() { "content-list" } else { "content-list light-mode" },

                        div {
                            class: "tag-group",
                            h3 { class: "tag-group-title", "🏷️ {active_tag_name}" }
                            for item in tag_content() {
                                ContentCard {
                                    item: item.clone(),
                                    content_tags_ctx: content_tags_ctx.clone(),
                                    on_card_select: move |content: ContentModel| {
                                        selected_content.set(Some(content));
                                        modal_open.set(true);
                                    },
                                }
                            }
                        }

                        if tag_content().is_empty() {
                            div {
                                class: if dark_mode() { "empty-state" } else { "empty-state light-mode" },
                                "No content available for this tag"
                            }
                        }
                    }
                }
            }

            Modal {
                is_open: modal_open,
                title: modal_title(),

                if let Some(content) = selected_content() {
                    ModalContentBody {
                        content: content.clone(),
                        tags_ctx: content_tags_ctx.clone(),
                    }
                }
            }
        }
    }
}
