use crate::components::{InterestCard, SkillTagsGroup};
use dioxus::prelude::*;

/// Interests page component displaying personal interests, goals, and skills
/// Uses dark mode context to theme the page appropriately
#[component]
pub fn Interests() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        section {
            class: if dark_mode() { "interests-section" } else { "interests-section light-mode" },
            h2 { "🌟 About Me & My Interests" }

            // About content section
            div {
                class: if dark_mode() { "about-content" } else { "about-content light-mode" },
                p { "I'm particularly interested in working with Rust and exploring its potential in building efficient and modern applications." }

                // Goals section
                div {
                    class: if dark_mode() { "goals" } else { "goals light-mode" },
                    h3 { "🚀 My Goals" }
                    ul {
                        li { "Build cool projects with Rust and Dioxus." }
                        li { "Simplify everyday tasks using ESP32 and automation tools." }
                        li { "Explore new ways to blend science and technology into meaningful creations." }
                    }
                }
            }

            // Interests grid section
            div {
                class: if dark_mode() { "interests-grid" } else { "interests-grid light-mode" },
                h3 { "🔥 Topics That Excite Me" }

                InterestCard {
                    icon: "🌿".to_string(),
                    title: "Planting".to_string(),
                    description: "Cultivating plants and creating green spaces.".to_string(),
                }

                InterestCard {
                    icon: "🤖".to_string(),
                    title: "Automating Workflows".to_string(),
                    description: "Streamlining processes and increasing efficiency through automation.".to_string(),
                }

                InterestCard {
                    icon: "🧠".to_string(),
                    title: "Science & Math".to_string(),
                    description: "Exploring the fundamentals of the natural world.".to_string(),
                }

                InterestCard {
                    icon: "⚙️".to_string(),
                    title: "ESP32 Projects".to_string(),
                    description: "Building IoT devices and embedded systems.".to_string(),
                }

                InterestCard {
                    icon: "💰".to_string(),
                    title: "Crypto".to_string(),
                    description: "Understanding blockchain technology and digital assets.".to_string(),
                }

                InterestCard {
                    icon: "🦀".to_string(),
                    title: "Rust".to_string(),
                    description: "Building safe and performant applications with Rust.".to_string(),
                }

                InterestCard {
                    icon: "🖥️".to_string(),
                    title: "Dioxus".to_string(),
                    description: "Creating cross-platform user interfaces with Rust.".to_string(),
                }
            }

            // Skills section
            div {
                class: if dark_mode() { "skills-section" } else { "skills-section light-mode" },
                h3 { "🔧 Skills" }

                // Programming Languages & Frameworks
                div {
                    class: if dark_mode() { "skill-category" } else { "skill-category light-mode" },
                    h4 { "Programming Languages & Frameworks" }
                    SkillTagsGroup {
                        skills: vec![
                            "Rust".to_string(),
                            "Go".to_string(),
                            "TypeScript".to_string(),
                            "Python".to_string(),
                            "JavaScript".to_string(),
                        ]
                    }
                }

                // Product Management & Testing
                div {
                    class: if dark_mode() { "skill-category" } else { "skill-category light-mode" },
                    h4 { "Product Management & Testing" }
                    SkillTagsGroup {
                        skills: vec![
                            "Product Management".to_string(),
                            "Test Planning".to_string(),
                            "Agile Methodologies".to_string(),
                            "Sprint Planning".to_string(),
                        ]
                    }
                }

                // Tools & Technologies
                div {
                    class: if dark_mode() { "skill-category" } else { "skill-category light-mode" },
                    h4 { "Tools & Technologies" }
                    SkillTagsGroup {
                        skills: vec![
                            "AWS".to_string(),
                            "VS Code".to_string(),
                            "TailwindCSS".to_string(),
                            "Figma".to_string(),
                            "Docker".to_string(),
                        ]
                    }
                }
            }
        }
    }
}
