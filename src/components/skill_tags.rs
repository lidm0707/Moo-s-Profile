use dioxus::prelude::*;

/// Individual skill tag component displaying a single skill
#[component]
pub fn SkillTag(skill: String) -> Element {
    rsx! {
        span { class: "skill-tag", "{skill}" }
    }
}

/// Skill tags group component displaying a collection of related skills
#[component]
pub fn SkillTagsGroup(skills: Vec<String>) -> Element {
    rsx! {
        div { class: "skill-tags",
            for skill in skills {
                SkillTag { skill: skill.clone() }
            }
        }
    }
}
