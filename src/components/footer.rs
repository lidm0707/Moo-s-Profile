use crate::components::Icon;
use chrono::Datelike;
use dioxus::prelude::*;

/// Footer component displaying copyright information
/// Uses dark mode context to theme the footer appropriately
#[component]
pub fn Footer() -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    let year = chrono::Utc::now().year();

    rsx! {
        footer {
            class: if dark_mode() { "profile-footer" } else { "profile-footer light-mode" },
            p {
                "© {year} Moo | Built with Rust and Dioxus "
                Icon { name: "crab".to_string(), class: "footer-icon" }
            }
        }
    }
}
