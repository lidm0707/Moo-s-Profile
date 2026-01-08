use dioxus::prelude::*;

/// Hook to get the current dark mode state from context
/// Returns an Option<Signal<bool>> since the context might not be provided
pub fn use_dark_mode_context() -> Option<Signal<bool>> {
    use_context()
}
