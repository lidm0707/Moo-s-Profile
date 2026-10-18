use content_sdk::models::{Content, ContentTag, Tag};
use dioxus::prelude::*;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::{JsFuture, spawn_local};

#[cfg(target_arch = "wasm32")]
const SNAPSHOT_URL: &str = "/content_snapshot.json";
const STATUS_PUBLISHED: &str = "published";
pub const LOADING: &str = "Loading…";

#[derive(serde::Deserialize, Default)]
pub struct Snapshot {
    #[serde(default)]
    content: Vec<Content>,
    #[serde(default)]
    tags: Vec<Tag>,
    #[serde(default)]
    content_tags: Vec<ContentTag>,
}

/// All Supabase data, loaded at boot from `/content_snapshot.json` instead of
/// being embedded in the wasm (embedding inflated the binary by the full
/// snapshot size, which dominated LCP on slow connections). Regenerate via
/// `make snapshot`; the file ships next to the app and is fetched in parallel
/// with the wasm via a preload link in `index.html`.
static SNAPSHOT: GlobalSignal<Option<Arc<Snapshot>>> = Signal::global(|| None);

fn parse(json: &str) -> Snapshot {
    serde_json::from_str(json).unwrap_or_default()
}

/// Kick off the snapshot load. Call once from `main` before launch.
pub fn init_fetch() {
    #[cfg(target_arch = "wasm32")]
    {
        spawn_local(async move {
            match fetch_text(SNAPSHOT_URL).await {
                Ok(json) => *SNAPSHOT.write() = Some(Arc::new(parse(&json))),
                Err(err) => {
                    web_sys::console::warn_1(&format!("[snapshot] load failed: {err}").into())
                }
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        *SNAPSHOT.write() = Some(Arc::new(parse(include_str!(
            "../assets/content_snapshot.json"
        ))));
    }
}

#[cfg(target_arch = "wasm32")]
async fn fetch_text(url: &str) -> Result<String, String> {
    let window = web_sys::window().ok_or("no window")?;
    let resp_val = JsFuture::from(window.fetch_with_str(url))
        .await
        .map_err(|e| format!("{e:?}"))?;
    let resp: web_sys::Response = resp_val.dyn_into().map_err(|e| format!("{e:?}"))?;
    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let text = JsFuture::from(resp.text().map_err(|e| format!("{e:?}"))?)
        .await
        .map_err(|e| format!("{e:?}"))?;
    Ok(text.as_string().unwrap_or_default())
}

/// True once the snapshot has arrived. Reading this inside render/effect
/// scopes subscribes the caller, so UI flips from loading to data on arrival.
pub fn loaded() -> bool {
    SNAPSHOT.read().is_some()
}

fn snapshot() -> Arc<Snapshot> {
    SNAPSHOT
        .read()
        .as_ref()
        .cloned()
        .unwrap_or_else(Arc::default)
}

pub fn all_tags() -> Vec<Tag> {
    snapshot().tags.clone()
}

pub fn content_by_ids(ids: &[i32]) -> Vec<Content> {
    snapshot()
        .content
        .iter()
        .filter(|c| c.status == STATUS_PUBLISHED && c.id.is_some_and(|id| ids.contains(&id)))
        .cloned()
        .collect()
}

pub fn all_published() -> Vec<Content> {
    snapshot()
        .content
        .iter()
        .filter(|c| c.status == STATUS_PUBLISHED)
        .cloned()
        .collect()
}

pub fn content_by_slug(slug: &str) -> Option<Content> {
    snapshot()
        .content
        .iter()
        .find(|c| c.status == STATUS_PUBLISHED && c.slug == slug)
        .cloned()
}

pub fn tags_for_content(content_id: i32) -> Vec<Tag> {
    let tag_ids: Vec<i32> = snapshot()
        .content_tags
        .iter()
        .filter(|ct| ct.content_id == content_id)
        .map(|ct| ct.tag_id)
        .collect();
    snapshot()
        .tags
        .iter()
        .filter(|t| t.id.is_some_and(|id| tag_ids.contains(&id)))
        .cloned()
        .collect()
}

pub fn content_ids_for_tag(tag_id: i32) -> Vec<i32> {
    snapshot()
        .content_tags
        .iter()
        .filter(|ct| ct.tag_id == tag_id)
        .map(|ct| ct.content_id)
        .collect()
}
