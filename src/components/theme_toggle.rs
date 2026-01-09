use dioxus::prelude::*;

/// Theme toggle component that switches between dark and light modes
/// Uses dark mode context to track and update the current theme
#[component]
pub fn ThemeToggle() -> Element {
    let mut dark_mode = use_context::<Signal<bool>>();
    dark_mode.set(true);
    rsx! {
        button {
            class: if dark_mode() { "theme-toggle" } else { "theme-toggle light-mode" },
            // onclick: move |_| {
            //     *dark_mode.write() = !dark_mode();
            // },
            if dark_mode() {
                "🌙"
            } else {
                "☀️"
            }
        }
    }
}
