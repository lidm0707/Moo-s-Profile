use content_sdk::models::{Content, ContentTag, Tag};
use std::sync::OnceLock;

const SNAPSHOT_JSON: &str = include_str!("../assets/content_snapshot.json");
const STATUS_PUBLISHED: &str = "published";

#[derive(serde::Deserialize, Default)]
pub struct Snapshot {
    #[serde(default)]
    content: Vec<Content>,
    #[serde(default)]
    tags: Vec<Tag>,
    #[serde(default)]
    content_tags: Vec<ContentTag>,
}

/// Embedded build-time copy of all Supabase data. The app never performs a
/// runtime API call; regenerate via `make snapshot` then rebuild.
fn snapshot() -> &'static Snapshot {
    static SNAP: OnceLock<Snapshot> = OnceLock::new();
    SNAP.get_or_init(|| serde_json::from_str(SNAPSHOT_JSON).unwrap_or_default())
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
