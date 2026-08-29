use dioxus::prelude::*;

use crate::routes::Route;

const HERO_IMAGE: Asset = asset!("/assets/profile.jpg");

const HERO_TITLE: &str = "Content Management System";
const HERO_ACCENT: &str = "Powered by Dioxus & Supabase";
const HERO_SUBTITLE: &str = "Create, edit, and manage your content with a calm, minimal interface. Built with Rust, Dioxus, and Supabase.";
const PRIMARY_CTA: &str = "Go to Dashboard";
const SECONDARY_CTA: &str = "Learn More";

#[component]
pub fn Home() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        section {
            class: if dark_mode() { "hero-section" } else { "hero-section light-mode" },

            div {
                class: "hero-content",
                h1 {
                    class: "hero-title",
                    "{HERO_TITLE}"
                    span { class: "hero-title-accent", "{HERO_ACCENT}" }
                }
                p { class: "hero-subtitle", "{HERO_SUBTITLE}" }
                div {
                    class: "hero-actions",
                    Link {
                        to: Route::ContentPage {},
                        class: "btn-primary",
                        "{PRIMARY_CTA}"
                    }
                    Link {
                        to: Route::Interests {},
                        class: "btn-secondary",
                        "{SECONDARY_CTA}"
                    }
                }
            }

            div {
                class: "hero-figure",
                img {
                    src: "{HERO_IMAGE}",
                    alt: "Profile",
                    class: "hero-image",
                }
            }
        }
    }
}
