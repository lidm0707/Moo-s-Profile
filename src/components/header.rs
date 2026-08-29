use crate::components::Icon;
use dioxus::prelude::*;

const PROFILE_IMAGE: Asset = asset!("/assets/profile.jpg");
const BG_IMAGE: Asset = asset!("/assets/bg_profile.jpg");

const EYEBROW: &str = "プロフィール · PROFILE";
const SIDE_TEXT_LEFT_JP: &str = "プロフィール";
const SIDE_TEXT_RIGHT_JP: &str = "モシネデイルナニ";
const TITLE_PLAIN: &str = "Hi there";
const TITLE_ACCENT: &str = "I'm Moo!";
const SUBTITLE_1: &str =
    "I'm passionate about creating, learning, and innovating in the field of technology.";
const SUBTITLE_2: &str =
    "I'm from Thailand, and I love diving into new challenges and opportunities.";

#[component]
pub fn Header() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        header {
            class: if dark_mode() { "profile-header" } else { "profile-header light-mode" },
            style: format!(
                "background-image:
                linear-gradient(
                  rgba(20, 16, 28, 0.88),
                  rgba(20, 16, 28, 0.92)
                ),url({});",
                BG_IMAGE
            ),

            div {
                class: if dark_mode() { "header-hero" } else { "header-hero light-mode" },

                aside {
                    class: "hero-side-text hero-side-left",
                    aria_hidden: true,
                    "{SIDE_TEXT_LEFT_JP}"
                }

                div {
                    class: "header-hero-content",
                    p { class: "hero-eyebrow", "{EYEBROW}" }
                    h1 {
                        class: "hero-title header-title",
                        span { "{TITLE_PLAIN}" }
                        span { class: "hero-title-accent", "{TITLE_ACCENT}" }
                    }
                    p { class: "hero-subtitle", "{SUBTITLE_1}" }
                    p { class: "hero-subtitle", "{SUBTITLE_2}" }
                    div {
                        class: "hero-actions",
                        a {
                            href: "https://github.com/lidm0707",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "btn-primary",
                            Icon { name: "github".to_string(), class: "social-icon" }
                            span { "GitHub" }
                        }
                        a {
                            href: "https://www.linkedin.com/in/kachon-wanglavan-4124a5216/",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "btn-secondary",
                            Icon { name: "linkedin".to_string(), class: "social-icon" }
                            span { "LinkedIn" }
                        }
                    }
                }

                div {
                    class: "hero-figure",
                    img {
                        src: "{PROFILE_IMAGE}",
                        alt: "Moo's profile picture",
                        class: "hero-image",
                    }
                }

                aside {
                    class: "hero-side-text hero-side-right",
                    aria_hidden: true,
                    "{SIDE_TEXT_RIGHT_JP}"
                }
            }
        }
    }
}
