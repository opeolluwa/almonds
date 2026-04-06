use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tag {
    Development,
    Inspiration,
    Design,
    Research,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Todo,
    Note,
    Reminder,
    Snippet,
    Bookmark,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemType::Todo => write!(f, "todo"),
            ItemType::Note => write!(f, "note"),
            ItemType::Reminder => write!(f, "reminder"),
            ItemType::Snippet => write!(f, "snippet"),
            ItemType::Bookmark => write!(f, "bookmark"),
            // ItemType::Workspace => write!(f, "workspace"),
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tag::Development => write!(f, "development"),
            Tag::Inspiration => write!(f, "inspiration"),
            Tag::Design => write!(f, "design"),
            Tag::Research => write!(f, "research"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::High => write!(f, "high"),
            Priority::Medium => write!(f, "medium"),
            Priority::Low => write!(f, "low"),
        }
    }
}
