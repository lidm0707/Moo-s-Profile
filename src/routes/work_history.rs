use crate::components::{Icon, WorkHistoryTimeline};
use dioxus::prelude::*;

/// Work history page component displaying professional experience
/// Uses dark mode context to theme the page appropriately
#[component]
pub fn WorkHistory() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        section {
            class: if dark_mode() { "work-history-section" } else { "work-history-section light-mode" },
            h2 {
                Icon { name: "briefcase".to_string(), class: "heading-icon" }
                span { "Work History" }
            }

            // Timeline component containing all work experience items
            WorkHistoryTimeline {}
        }
    }
}
