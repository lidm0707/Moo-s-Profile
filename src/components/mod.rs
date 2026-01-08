// Re-export all components for easy importing
pub mod footer;
pub mod header;
pub mod interest_card;
pub mod nav;
pub mod skill_tags;
pub mod theme_toggle;
pub mod timeline;

// Re-export commonly used components at the module level
pub use footer::Footer;
pub use header::Header;
pub use interest_card::InterestCard;
pub use nav::Nav;
pub use skill_tags::SkillTagsGroup;
pub use theme_toggle::ThemeToggle;
pub use timeline::WorkHistoryTimeline;
