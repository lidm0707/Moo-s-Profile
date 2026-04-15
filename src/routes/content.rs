use crate::routes::Route;
use chrono::Utc;
use content_sdk::contexts::{ContentContext, ContentTagsContext, TagContext};
use content_sdk::models::Content as ContentModel;
use content_sdk::utils::render_markdown_to_html;

use dioxus::prelude::*;

/// Content page component displaying all content in vertical list
/// Uses pagination for navigating through content items
/// Fetches content, tags, and content-tag relationships from Supabase
#[component]
pub fn ContentPage() -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    let mut current_page = use_signal(|| 1u32);
    const ITEMS_PER_PAGE: u32 = 5;

    let content_ctx = use_context::<ContentContext>();
    let tag_ctx = use_context::<TagContext>();
    let content_tags_ctx = use_context::<ContentTagsContext>();

    // Fetch paginated content using SDK
    let content_resource = use_resource(move || {
        let ctx = content_ctx.clone();
        let page = *current_page.read();
        async move {
            match ctx.get_paginated_content(&[], page, ITEMS_PER_PAGE).await {
                Ok(response) => {
                    info!(
                        "📦 Fetched {} items (page {} of {}, total: {})",
                        response.data.len(),
                        response.page,
                        response.total_pages,
                        response.total_items
                    );
                    for (idx, item) in response.data.iter().enumerate() {
                        info!("  Item {}: '{}' (id: {:?})", idx, item.title, item.id);
                    }
                    Ok(response)
                }
                Err(e) => {
                    error!("❌ Failed to fetch paginated content: {}", e);
                    Err(e)
                }
            }
        }
    });

    // Fetch all tags
    let tags_resource = use_resource(move || {
        let ctx = tag_ctx.clone();
        async move { ctx.get_all_tags().await }
    });

    // Get paginated data from SDK response
    let paginated_data = use_memo(move || -> Option<(u32, u32, Vec<ContentModel>)> {
        match content_resource.read().as_ref() {
            Some(Ok(response)) => Some((
                response.total_items,
                response.total_pages,
                response.data.clone(),
            )),
            _ => None,
        }
    });

    let is_loading = use_memo(move || content_resource().is_none() || tags_resource().is_none());

    // Determine if there's an error
    let has_error = use_memo(move || matches!(content_resource().as_ref(), Some(Err(_))));

    // Determine if content is empty
    let is_empty = use_memo(
        move || matches!(content_resource().as_ref(), Some(Ok(response)) if response.data.is_empty()),
    );

    // Log pagination state for debugging
    let _log_pagination = move || {
        if let Some((_total_items, total_pages, _)) = paginated_data().as_ref()
            && *total_pages > 1
        {
            info!(
                "🎯 Showing pagination controls: {} pages total",
                total_pages
            );
        }
    };

    rsx! {
        section {
            class: if dark_mode() { "content-section" } else { "content-section light-mode" },
            h2 { "📚 Content" }

            // Loading state
            if *is_loading.read() {
                div {
                    class: if dark_mode() { "loading" } else { "loading light-mode" },
                    "Loading content..."
                }
            }

            // Error state
            else if *has_error.read() {
                if let Some(Err(e)) = content_resource() {
                    div {
                        class: if dark_mode() { "error-state" } else { "error-state light-mode" },
                        "Error loading content: {e}"
                    }
                }
            }

            // Empty state
            else if *is_empty.read() {
                div {
                    class: if dark_mode() { "empty-state" } else { "empty-state light-mode" },
                    "No content available"
                }
            }

            // Content list
            else {
                div {
                    class: if dark_mode() { "content-list" } else { "content-list light-mode" },
                    for item in paginated_data().as_ref().map(|(_, _, data)| data).unwrap_or(&vec![]) {
                        ContentCard {
                            item: item.clone(),
                            content_tags_ctx: content_tags_ctx.clone(),
                        }
                    }
                }

                // Pagination controls
                if let Some((_, total_pages, _)) = paginated_data().as_ref() {
                    if *total_pages > 1 {
                        PaginationControls {
                            current_page: current_page(),
                            total_pages: *total_pages,
                            dark_mode,
                            on_page_change: move |page| {
                                info!("🔄 User clicked to navigate to page {}", page);
                                current_page.set(page);
                            },
                        }
                    }
                }
            }
        }
    }
}

/// PaginationControls component for navigating between pages
#[component]
fn PaginationControls(
    current_page: u32,
    total_pages: u32,
    dark_mode: Signal<bool>,
    on_page_change: EventHandler<u32>,
) -> Element {
    let prev_disabled = current_page == 1;
    let next_disabled = current_page == total_pages;

    rsx! {
        div {
            class: if dark_mode() { "pagination" } else { "pagination light-mode" },

            button {
                class: if dark_mode() { "pagination-button" } else { "pagination-button light-mode" },
                disabled: prev_disabled,
                onclick: move |_| {
                    if current_page > 1 {
                        on_page_change(current_page - 1);
                    }
                },
                "← Previous"
            }

            for page in 1..=total_pages {
                button {
                    class: {
                        let base_class = if dark_mode() { "pagination-button" } else { "pagination-button light-mode" };
                        if page == current_page {
                            format!("{} {}", base_class, "pagination-button-active")
                        } else {
                            base_class.to_string()
                        }
                    },
                    onclick: move |_| {
                        on_page_change(page);
                    },
                    "{page}"
                }
            }

            button {
                class: if dark_mode() { "pagination-button" } else { "pagination-button light-mode" },
                disabled: next_disabled,
                onclick: move |_| {
                    if current_page < total_pages {
                        on_page_change(current_page + 1);
                    }
                },
                "Next →"
            }
        }
    }
}

/// ContentCard component displaying a single content item with its tags
#[component]
fn ContentCard(item: ContentModel, content_tags_ctx: ContentTagsContext) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    // Fetch tags for this content item
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

    // Get tags or empty
    let tags_list = use_memo(move || tags_resource().clone().unwrap_or_default());

    // Determine if tags are loading
    let tags_loading = use_memo(move || tags_resource().is_none());

    let created_at = item
        .created_at
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    rsx! {
        article {
            class: if dark_mode() { "content-card" } else { "content-card light-mode" },
            header {
                class: "content-card-header",
                h3 { class: "content-card-title", "{item.title}" }
                time { class: "content-card-date", "{created_at}" }
            }
            div {
                class: "content-card-tags",
                // Render tags when loaded
                if !*tags_loading.read() {
                    for tag in tags_list() {
                        span {
                            class: "content-tag",
                            "{tag.name}"
                        }
                    }
                }
            }
            Link {
                to: Route::ContentDetail { slug: item.slug.clone() },
                button {
                    class: if dark_mode() { "content-card-button" } else { "content-card-button light-mode" },
                    "Read More →"
                }
            }
        }
    }
}

/// ContentDetail component displaying a single content item in full
#[component]
pub fn ContentDetail(slug: String) -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    let content_ctx = use_context::<ContentContext>();
    let content_tags_ctx = use_context::<ContentTagsContext>();

    // Fetch content by slug
    let content_resource = use_resource(move || {
        let ctx = content_ctx.clone();
        let slug_clone = slug.clone();
        async move { ctx.get_content_by_slug(&slug_clone).await }
    });

    // Fetch tags for this content
    let tags_resource = use_resource(move || {
        let ctx = content_tags_ctx.clone();
        let content_id = match content_resource.read().as_ref() {
            Some(Ok(Some(content))) => content.id,
            _ => None,
        };
        async move {
            if let Some(id) = content_id {
                ctx.get_tags_for_content(id).await.unwrap_or_default()
            } else {
                vec![]
            }
        }
    });

    // Determine if loading
    let is_loading = use_memo(move || content_resource.read().is_none());

    rsx! {
        section {
            class: if dark_mode() { "content-section" } else { "content-section light-mode" },

            // Back button
            div {
                class: "content-detail-header",
                Link {
                    to: Route::ContentPage {},
                    button {
                        class: if dark_mode() { "content-detail-back" } else { "content-detail-back light-mode" },
                        "← Back to Content"
                    }
                }
            }

            // Loading state
            if *is_loading.read() {
                div {
                    class: if dark_mode() { "loading" } else { "loading light-mode" },
                    "Loading content..."
                }
            } else {
                match content_resource.read().as_ref() {
                    Some(Err(e)) => rsx! {
                        div {
                            class: if dark_mode() { "error-state" } else { "error-state light-mode" },
                            "Error loading content: {e}"
                        }
                    },
                    Some(Ok(None)) => rsx! {
                        div {
                            class: if dark_mode() { "empty-state" } else { "empty-state light-mode" },
                            "Content not found"
                        }
                    },
                    Some(Ok(Some(content))) => rsx! {
                        article {
                            class: if dark_mode() { "content-detail" } else { "content-detail light-mode" },
                            header {
                                class: "content-detail-header",
                                h1 { class: "content-detail-title", "{content.title}" }

                            }
                            time {
                                class: "content-detail-date",
                                {
                                    content.created_at
                                        .map(|dt| dt.format("%B %e, %Y").to_string())
                                        .unwrap_or_else(|| Utc::now().format("%B %e, %Y").to_string())
                                }
                            }
                            div {
                                class: "content-detail-body",
                                dangerous_inner_html: "{render_markdown_to_html(&content.body)}"
                            }
                            div {
                                class: "content-detail-tags",
                                match tags_resource.read().as_ref() {
                                    Some(tags) => rsx! {
                                        for tag in tags {
                                            span {
                                                class: "content-tag",
                                                "{tag.name}"
                                            }
                                        }
                                    },
                                    None => rsx! { "" },
                                }
                            }
                        }
                    },
                    _ => rsx! { "" },
                }
            }
        }
    }
}

/// Generates an excerpt from content body
fn generate_excerpt(body: &str) -> String {
    let plain_text = body
        .replace(['#', '*', '_'], "")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join(" ");

    if plain_text.chars().count() > 150 {
        let end_byte = plain_text
            .char_indices()
            .nth(150)
            .map(|(idx, _)| idx)
            .unwrap_or(plain_text.len());
        format!("{}...", &plain_text[..end_byte].trim_end())
    } else {
        plain_text
    }
}
