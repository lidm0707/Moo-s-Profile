use crate::components::{Footer, Header, Nav, ThemeToggle};
use dioxus::prelude::*;

/// Route definitions for the application
/// All routes are defined here with their paths and layout
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // Apply ProfileLayout to all routes
    #[layout(ProfileLayout)]

    // Home route displays Interests page
    #[route("/")]
    Interests {},

    // Work history route
    #[route("/work-history")]
    WorkHistory {},
}

/// ProfileLayout component that wraps all pages with common UI elements
/// This includes theme toggle, header, navigation, and footer
/// Provides dark mode context to all child components
#[component]
pub fn ProfileLayout() -> Element {
    let mut dark_mode = use_signal(|| true);
    use_context_provider(|| dark_mode);

    rsx! {
        // Theme toggle button
        ThemeToggle {}

        // Header with profile information
        Header {}

        // Navigation menu
        Nav {}

        // Main content area where child routes are rendered
        main {
            class: if dark_mode() { "profile-content" } else { "profile-content light-mode" },
            Outlet::<Route> {}
        }

        // Footer with copyright information
        Footer {}
    }
}

// Re-export route components for easy importing
pub use interests::Interests;
pub use work_history::WorkHistory;

// Import the individual route page components
mod interests;
mod work_history;
