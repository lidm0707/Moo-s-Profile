use dioxus::prelude::*;

/// Timeline component displaying work history as a vertical timeline
/// Uses dark mode context to theme the timeline appropriately
#[component]
pub fn Timeline(children: Element) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        div {
            class: if dark_mode() { "timeline" } else { "timeline light-mode" },
            {children}
        }
    }
}

/// Individual timeline item component representing a single job/position
/// Uses dark mode context to theme the item appropriately
#[component]
pub fn TimelineItem(
    title: String,
    date: String,
    description: String,
    achievements: Vec<String>,
) -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        div {
            class: if dark_mode() { "timeline-item" } else { "timeline-item light-mode" },

            TimelineDot {},

            div {
                class: if dark_mode() { "timeline-content" } else { "timeline-content light-mode" },
                h3 { "{title}" }
                div {
                    class: "timeline-date",
                    "{date}"
                }
                p { "{description}" }
                if !achievements.is_empty() {
                    ul {
                        for achievement in achievements {
                            li { "{achievement}" }
                        }
                    }
                }
            }
        }
    }
}

/// Timeline dot component representing the visual marker on the timeline
/// Uses dark mode context to theme the dot appropriately
#[component]
pub fn TimelineDot() -> Element {
    let dark_mode = use_context::<Signal<bool>>();

    rsx! {
        div {
            class: if dark_mode() { "timeline-dot" } else { "timeline-dot light-mode" }
        }
    }
}

/// Work history timeline component that composes all work experience items
/// This is a high-level component that organizes all work history data
#[component]
pub fn WorkHistoryTimeline() -> Element {
    rsx! {
        Timeline {
            TimelineItem {
                title: "General Manager".to_string(),
                date: "Jan 2022 – Present · 3 years 11 months".to_string(),
                description: "Leading company-wide digital transformation initiatives and IT strategy management, with a focus on operational efficiency and technology adoption.".to_string(),
                achievements: vec![
                    "Spearheaded company-wide digital transformation projects, improving operational workflows and collaboration".to_string(),
                    "Automated and streamlined processes across departments, boosting efficiency and reducing repetitive tasks".to_string(),
                    "Defined and managed IT strategies, budgets, and KPIs, enabling more informed decision-making".to_string(),
                    "Partnered with cross-functional teams to ensure smooth adoption of new systems and technologies".to_string(),
                ],
            }
            TimelineItem {
                title: "Product Owner".to_string(),
                date: "Dec 2018 – Dec 2021 · 3 years 1 month".to_string(),
                description: "Managing product roadmap and delivery lifecycle, bridging business objectives with technical implementation and user needs.".to_string(),
                achievements: vec![
                    "Orchestrated product roadmap and backlog to align with business objectives and customer needs".to_string(),
                    "Collaborated with developers, designers, and QA to deliver high-quality product features".to_string(),
                    "Conducted market research and user interviews to guide feature development".to_string(),
                    "Facilitated sprint planning, reviews, and retrospectives to continuously improve team performance".to_string(),
                ],
            }
        }
    }
}
