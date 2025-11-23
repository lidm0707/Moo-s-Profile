use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(ProfileLayout)]
    #[route("/")]
    Interests {},
    #[route("/work-history")]
    WorkHistory {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}

#[component]
fn ProfileLayout() -> Element {
    let mut dark_mode = use_signal(|| true);
    use_context_provider(|| dark_mode);

    rsx! {
        // Theme toggle button
        button {
            class: if dark_mode() { "theme-toggle" } else { "theme-toggle light-mode" },
            onclick: move |_| {
                *dark_mode.write() = !dark_mode();
            },
            if dark_mode() {
                "🌙"
            } else {
                "☀️"
            }
        }

        // Header with profile information
        header {
            class: if dark_mode() { "profile-header" } else { "profile-header light-mode" },
            div {
                class: "profile-info",
                h1 { "Hi there 👋 I'm Moo!" }
                p { "I'm passionate about creating, learning, and innovating in the field of technology." }
                p { "I'm from Thailand, and I love diving into new challenges and opportunities." }

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

        // Navigation
        nav {
            class: if dark_mode() { "profile-nav" } else { "profile-nav light-mode" },
            Link {
                to: Route::Interests {},
                class: "nav-link",
                id: "interests-link",
                "My Interests"
            }
            Link {
                to: Route::WorkHistory {},
                class: "nav-link",
                id: "work-history-link",
                "Work History"
            }
        }

        // Main content area
        main {
            class: if dark_mode() { "profile-content" } else { "profile-content light-mode" },
            Outlet::<Route> {}
        }

        // Footer
        footer {
            class: if dark_mode() { "profile-footer" } else { "profile-footer light-mode" },
            p { "© 2023 Moo | Built with Rust and Dioxus 🦀" }
        }
    }
}

#[component]
fn Interests() -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    rsx! {
        section {
            class: if dark_mode() { "interests-section" } else { "interests-section light-mode" },
            h2 { "🌟 About Me & My Interests" }

            div {
                class: if dark_mode() { "about-content" } else { "about-content light-mode" },
                p { "I'm particularly interested in working with Rust and exploring its potential in building efficient and modern applications." }

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

            div {
                class: if dark_mode() { "interests-grid" } else { "interests-grid light-mode" },
                h3 { "🔥 Topics That Excite Me" }

                div {
                    class: if dark_mode() { "interest-card" } else { "interest-card light-mode" },
                    h4 { "🌿 Planting" }
                    p { "Cultivating plants and creating green spaces." }
                }

                div {
                    class: if dark_mode() { "interest-card" } else { "interest-card light-mode" },
                    h4 { "🤖 Automating Workflows" }
                    p { "Streamlining processes and increasing efficiency through automation." }
                }

                div {
                    class: if dark_mode() { "interest-card" } else { "interest-card light-mode" },
                    h4 { "🧠 Science & Math" }
                    p { "Exploring the fundamentals of the natural world." }
                }

                div {
                    class: if dark_mode() { "interest-card" } else { "interest-card light-mode" },
                    h4 { "⚙️ ESP32 Projects" }
                    p { "Building IoT devices and embedded systems." }
                }

                div {
                    class: if dark_mode() { "interest-card" } else { "interest-card light-mode" },
                    h4 { "💰 Crypto" }
                    p { "Understanding blockchain technology and digital assets." }
                }

                div {
                    class: if dark_mode() { "interest-card" } else { "interest-card light-mode" },
                    h4 { "🦀 Rust" }
                    p { "Building safe and performant applications with Rust." }
                }

                div {
                    class: "interest-card",
                    h4 { "🖥️ Dioxus" }
                    p { "Creating cross-platform user interfaces with Rust." }
                }
            }

            div {
                class: if dark_mode() { "skills-section" } else { "skills-section light-mode" },
                h3 { "🔧 Skills" }

                div {
                    class: if dark_mode() { "skill-category" } else { "skill-category light-mode" },
                    h4 { "Programming Languages & Frameworks" }
                    div {
                        class: "skill-tags",
                        span { class: "skill-tag", "Rust" }
                        span { class: "skill-tag", "Go" }
                        span { class: "skill-tag", "TypeScript" }
                        span { class: "skill-tag", "Python" }
                        span { class: "skill-tag", "JavaScript" }
                    }
                }

                div {
                    class: if dark_mode() { "skill-category" } else { "skill-category light-mode" },
                    h4 { "Product Management & Testing" }
                    div {
                        class: "skill-tags",
                        span { class: "skill-tag", "Product Management" }
                        span { class: "skill-tag", "Test Planning" }
                        span { class: "skill-tag", "Agile Methodologies" }
                        span { class: "skill-tag", "Sprint Planning" }
                    }
                }

                div {
                    class: if dark_mode() { "skill-category" } else { "skill-category light-mode" },
                    h4 { "Tools & Technologies" }
                    div {
                        class: "skill-tags",
                        span { class: "skill-tag", "AWS" }
                        span { class: "skill-tag", "VS Code" }
                        span { class: "skill-tag", "TailwindCSS" }
                        span { class: "skill-tag", "Figma" }
                        span { class: "skill-tag", "Docker" }
                    }
                }
            }
        }
    }
}

#[component]
fn WorkHistory() -> Element {
    let dark_mode = use_context::<Signal<bool>>();
    rsx! {
        section {
            class: if dark_mode() { "work-history-section" } else { "work-history-section light-mode" },
            h2 { "💼 Work History" }

        div {
            class: if dark_mode() { "timeline" } else { "timeline light-mode" },

            div {
                class: if dark_mode() { "timeline-item" } else { "timeline-item light-mode" },
                div {
                    class: if dark_mode() { "timeline-dot" } else { "timeline-dot light-mode" }
                }
                div {
                    class: if dark_mode() { "timeline-content" } else { "timeline-content light-mode" },
                    h3 { "General Manager" }
                    div {
                        class: "timeline-date",
                        "ม.ค. 2022 - ปัจจุบัน · 3 ปี 11 เดือน"
                    }
                    p { "Leading company-wide digital transformation initiatives and IT strategy management, with a focus on operational efficiency and technology adoption." }
                    ul {
                        li { "Spearheaded company-wide digital transformation projects, improving operational workflows and collaboration" }
                        li { "Automated and streamlined processes across departments, boosting efficiency and reducing repetitive tasks" }
                        li { "Defined and managed IT strategies, budgets, and KPIs, enabling more informed decision-making" }
                        li { "Partnered with cross-functional teams to ensure smooth adoption of new systems and technologies" }
                    }
                }
            }

            div {
                class: if dark_mode() { "timeline-item" } else { "timeline-item light-mode" },
                div {
                    class: if dark_mode() { "timeline-dot" } else { "timeline-dot light-mode" }
                }
                div {
                    class: if dark_mode() { "timeline-content" } else { "timeline-content light-mode" },
                    h3 { "Product Owner" }
                    div {
                        class: "timeline-date",
                        "ธ.ค. 2018 - ธ.ค. 2021 · 3 ปี 1 เดือน"
                    }
                    p { "Managing product roadmap and delivery lifecycle, bridging business objectives with technical implementation and user needs." }
                    ul {
                        li { "Orchestrated product roadmap and backlog to align with business objectives and customer needs" }
                        li { "Collaborated with developers, designers, and QA to deliver high-quality product features" }
                        li { "Conducted market research and user interviews to guide feature development" }
                        li { "Facilitated sprint planning, reviews, and retrospectives to continuously improve team performance" }
                    }
                }
            }
        }
        }
    }
}
