use crate::routes::Route;
use dioxus::prelude::*;

/// Navigation component providing links to different sections
/// Uses dark mode context to theme the navigation appropriately
#[component]
pub fn Nav() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        nav {
            class: if dark_mode() { "profile-nav" } else { "profile-nav light-mode" },
            Link {
                to: Route::Interests {},
                class: "nav-link",
                id: "interests-link",
                "My Interests"
            }
            Link {
                to: Route::WorkHistory {},
                class: "nav-link",
                id: "work-history-link",
                "Work History"
            }
        }
    }
}
