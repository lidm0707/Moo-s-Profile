use dioxus::prelude::*;

/// Inline SVG icon set (Lucide-style, 24x24 stroke icons).
/// Usage: `Icon { name: "flame".to_string() }`
#[component]
pub fn Icon(name: String, class: Option<String>) -> Element {
    rsx! {
        svg {
            class: class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            {icon_shapes(&name)}
        }
    }
}

fn icon_shapes(name: &str) -> Element {
    match name {
        "star" => rsx! {
            path { d: "M12 2l3.09 6.26L22 9.27l-5 4.87L18.18 21 12 17.77 5.82 21 7 14.14l-5-4.87 6.91-1.01L12 2z" }
        },
        "rocket" => rsx! {
            path { d: "M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z" }
            path { d: "m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z" }
            path { d: "M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0" }
            path { d: "M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5" }
        },
        "flame" => rsx! {
            path { d: "M8.5 14.5A2.5 2.5 0 0 0 11 12c0-1.38-.5-2-1-3-1.072-2.143-.224-4.054 2-6 .5 2.5 2 4.9 4 6.5 2 1.6 3 3.5 3 5.5a7 7 0 1 1-14 0c0-1.153.433-2.294 1-3a2.5 2.5 0 0 0 2.5 2.5z" }
        },
        "wrench" => rsx! {
            path { d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" }
        },
        "briefcase" => rsx! {
            rect { x: "2", y: "7", width: "20", height: "14", rx: "2" }
            path { d: "M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" }
        },
        "leaf" => rsx! {
            path { d: "M11 20A7 7 0 0 1 9.8 6.1C15.5 5 17 4.48 19 2c1 2 2 4.18 2 8 0 5.5-4.78 10-10 10Z" }
            path { d: "M2 21c0-3 1.85-5.36 5.08-6C9.5 14.52 12 13 13 12" }
        },
        "zap" => rsx! {
            path { d: "M13 2 3 14h9l-1 8 10-12h-9l1-8z" }
        },
        "brain" => rsx! {
            path { d: "M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z" }
            path { d: "M12 5a3 3 0 1 1 5.997.125 4 4 0 0 1 2.526 5.77 4 4 0 0 1-.556 6.588A4 4 0 1 1 12 18Z" }
        },
        "cpu" => rsx! {
            rect { x: "4", y: "4", width: "16", height: "16", rx: "2" }
            rect { x: "9", y: "9", width: "6", height: "6" }
            path { d: "M9 1v3M15 1v3M9 20v3M15 20v3M1 9h3M1 15h3M20 9h3M20 15h3" }
        },
        "coin" => rsx! {
            circle { cx: "12", cy: "12", r: "10" }
            path { d: "M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8" }
            path { d: "M12 18V6" }
        },
        "crab" => rsx! {
            path { d: "M9 11a3 3 0 1 1 6 0" }
            path { d: "M7.5 14a4.5 4.5 0 0 1 9 0v1a4.5 4.5 0 0 1-9 0z" }
            path { d: "M7 9 4 6M4 6a2 2 0 1 0-2.83 0M17 9l3-3M20 6a2 2 0 1 1 2.83 0" }
            path { d: "M8 17c-1.5.5-3 .5-4.5-.5M16 17c1.5.5 3 .5 4.5-.5M9.5 19 8 22M14.5 19l1.5 3" }
        },
        "monitor" => rsx! {
            rect { x: "2", y: "3", width: "20", height: "14", rx: "2" }
            path { d: "M8 21h8M12 17v4" }
        },
        "code" => rsx! {
            path { d: "m16 18 6-6-6-6M8 6l-6 6 6 6" }
        },
        "tag" => rsx! {
            path { d: "M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z" }
            circle { cx: "7.5", cy: "7.5", r: "0.5", fill: "currentColor" }
        },
        "moon" => rsx! {
            path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
        },
        "sun" => rsx! {
            circle { cx: "12", cy: "12", r: "4" }
            path { d: "M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" }
        },
        "calculator" => rsx! {
            rect { x: "4", y: "2", width: "16", height: "20", rx: "2" }
            path { d: "M8 6h8M16 14v4M12 10h.01M12 14h.01M12 18h.01M8 10h.01M8 14h.01M8 18h.01M16 10h.01" }
        },
        "github" => rsx! {
            path { d: "M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" }
            path { d: "M9 18c-4.51 2-5-2-7-2" }
        },
        "linkedin" => rsx! {
            path { d: "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" }
            rect { x: "2", y: "9", width: "4", height: "12" }
            circle { cx: "4", cy: "4", r: "2" }
        },
        _ => rsx! { circle { cx: "12", cy: "12", r: "10" } },
    }
}
