use content_sdk::contexts::TagContext;
use content_sdk::models::Tag;
use dioxus::prelude::*;

pub fn use_tags(tag_ctx: TagContext) -> Resource<Vec<Tag>> {
    use_resource(move || {
        let ctx = tag_ctx.clone();
        async move {
            match ctx.get_all_tags().await {
                Ok(tags) => tags,
                Err(e) => {
                    error!("Failed to fetch tags: {e}");
                    vec![]
                }
            }
        }
    })
}
