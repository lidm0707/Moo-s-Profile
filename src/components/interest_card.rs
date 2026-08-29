use crate::components::Icon;
use dioxus::prelude::*;

/// Interest card component displaying a single interest with icon and description
/// Uses dark mode context to theme the card appropriately
#[component]
pub fn InterestCard(icon: String, title: String, description: String) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        div {
            class: if dark_mode() { "interest-card" } else { "interest-card light-mode" },
            h4 {
                Icon { name: icon.clone(), class: "interest-card-icon" }
                span { "{title}" }
            }
            p { "{description}" }
        }
    }
}
