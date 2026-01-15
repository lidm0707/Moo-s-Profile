use dioxus::prelude::*;

/// Header component displaying profile information and social links
/// Uses dark mode context to theme the header appropriately
#[component]
pub fn Header() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        header {
            class: if dark_mode() { "profile-header" } else { "profile-header light-mode" },
            style: format!(
                "background-image:
                linear-gradient(
                  rgba(10, 10, 20, 0.85),
                  rgba(10, 10, 20, 0.85)
                ),url({});",
                asset!("/assets/bg_profile.jpg")
            ),
            div {
                class: "profile-info ",
                div{
                    img {
                    src: asset!("/assets/profile.jpg"),
                    alt: "Moo's profile picture",
                    class: "profile-image",
                }
                }
                div{class:"glow-text",
                    h1 { "Hi there I'm Moo!" }
                    p { "I'm passionate about creating, learning, and innovating in the field of technology." }
                    p { "I'm from Thailand, and I love diving into new challenges and opportunities." }
                }




                // Social links
                div {
                    class: "social-links",
                    a {
                        href: "https://github.com/lidm0707",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "GitHub"
                    }
                    a {
                        href: "https://www.linkedin.com/in/kachon-wanglavan-4124a5216/",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "LinkedIn"
                    }
                }
            }
        }
    }
}
