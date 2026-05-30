use crate::components::{Footer, Header, Nav, ThemeToggle};
use content_sdk::contexts::{ContentContext, ContentTagsContext, TagContext};
use content_sdk::utils::config::Config;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(ProfileLayout)]
    #[route("/")]
    Home {},
    #[route("/interests")]
    Interests {},
    #[route("/work-history")]
    WorkHistory {},
    #[route("/content")]
    ContentPage {},
    #[route("/content/:slug")]
    ContentDetail { slug: String },
}

#[component]
fn Home() -> Element {
    let nav = navigator();
    use_hook(move || nav.push(Route::ContentPage {}));
    rsx! {}
}

#[component]
pub fn ProfileLayout() -> Element {
    let dark_mode = use_signal(|| true);
    use_context_provider(|| dark_mode);

    let route: Route = use_route();
    let is_content = matches!(route, Route::ContentPage {} | Route::ContentDetail { .. });

    let config = use_hook(|| {
        let mode = env!("APP_MODE");
        let supabase_url = env!("SUPABASE_URL");
        let supabase_anon_key = env!("SUPABASE_ANON_KEY");

        Config::new(mode, supabase_url, supabase_anon_key, None)
    });

    use_context_provider(|| ContentContext::new(Some(config.clone())));
    use_context_provider(|| TagContext::new(Some(config.clone())));
    use_context_provider(|| ContentTagsContext::new(Some(config)));

    if is_content {
        rsx! {
            Nav {}
            div {
                class: if dark_mode() { "content-layout" } else { "content-layout light-mode" },
                Outlet::<Route> {}
            }
        }
    } else {
        rsx! {
            ThemeToggle {}
            Header {}
            Nav {}
            main {
                class: if dark_mode() { "profile-content" } else { "profile-content light-mode" },
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}

pub use content::{ContentDetail, ContentPage};
pub use interests::Interests;
pub use work_history::WorkHistory;

mod content;
mod interests;
mod work_history;
