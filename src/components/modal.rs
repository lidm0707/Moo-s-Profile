use dioxus::prelude::*;

#[component]
pub fn Modal(is_open: Signal<bool>, title: String, children: Element) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    if !is_open() {
        return rsx! {};
    }

    rsx! {
        div {
            class: if dark_mode() { "modal-backdrop" } else { "modal-backdrop light-mode" },
            onclick: move |_| is_open.set(false),

            div {
                class: if dark_mode() { "modal-container" } else { "modal-container light-mode" },
                onclick: move |e| e.stop_propagation(),

                header {
                    class: "modal-header",
                    h3 { class: "modal-title", "{title}" }
                    button {
                        class: "modal-close",
                        onclick: move |_| is_open.set(false),
                        "×"
                    }
                }

                div {
                    class: "modal-body",
                    {children}
                }
            }
        }
    }
}
