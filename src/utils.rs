use chrono::{Datelike, Utc};
use dioxus::prelude::*;
use wasm_bindgen::JsCast;

const HTTP_PREFIX: &str = "http";
const ANCHOR_SELECTOR: &str = "a";
const NEW_TAB_TARGET: &str = "_blank";

/// Click handler for rendered markdown bodies: opens external links in a new
/// tab; relative links and `#anchors` keep the browser's default behavior.
pub fn open_external_links_in_new_tab(event: Event<MouseData>) {
    let mouse_data = event.data();
    let Some(mouse_event) = mouse_data.downcast::<web_sys::MouseEvent>() else {
        return;
    };
    let Some(target) = mouse_event.target() else {
        return;
    };
    let Ok(element) = target.dyn_into::<web_sys::Element>() else {
        return;
    };
    let Some(anchor) = element.closest(ANCHOR_SELECTOR).ok().flatten() else {
        return;
    };
    let Some(href) = anchor.get_attribute("href") else {
        return;
    };
    // Only absolute http(s) links are external; `#fragments`, `/routes`,
    // `mailto:` etc. keep their default behavior.
    if !href.starts_with(HTTP_PREFIX) {
        return;
    }
    let same_origin = web_sys::window()
        .and_then(|w| w.location().origin().ok())
        .is_some_and(|origin| href.contains(&origin));
    if same_origin {
        return;
    }
    if let Some(window) = web_sys::window() {
        let _ = window.open_with_url_and_target(&href, NEW_TAB_TARGET);
        event.prevent_default();
    }
}

/// Best-effort clipboard write (secure contexts only; failures are silent).
pub fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        let _ = window.navigator().clipboard().write_text(text);
    }
}

/// Human duration between a month and today, e.g. "3 years 11 months".
pub fn duration_since(start_year: i32, start_month: u32) -> String {
    let now = Utc::now();
    let months = (now.year() - start_year) * 12 + now.month() as i32 - start_month as i32;
    format_months(months.max(0) as u32)
}

fn format_months(total: u32) -> String {
    let years = total / 12;
    let months = total % 12;
    match (years, months) {
        (0, m) => format!("{} month{}", m, if m == 1 { "" } else { "s" }),
        (y, 0) => format!("{} year{}", y, if y == 1 { "" } else { "s" }),
        (y, m) => format!(
            "{} year{} {} month{}",
            y,
            if y == 1 { "" } else { "s" },
            m,
            if m == 1 { "" } else { "s" }
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::format_months;

    #[test]
    fn plural_forms() {
        assert_eq!(format_months(47), "3 years 11 months");
        assert_eq!(format_months(37), "3 years 1 month");
        assert_eq!(format_months(12), "1 year");
        assert_eq!(format_months(1), "1 month");
        assert_eq!(format_months(25), "2 years 1 month");
    }
}
