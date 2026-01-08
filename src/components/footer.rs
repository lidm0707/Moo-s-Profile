use dioxus::prelude::*;

/// Footer component displaying copyright information
/// Uses dark mode context to theme the footer appropriately
#[component]
pub fn Footer() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        footer {
            class: if dark_mode() { "profile-footer" } else { "profile-footer light-mode" },
            p { "© 2025 Moo | Built with Rust and Dioxus 🦀" }
        }
    }
}
