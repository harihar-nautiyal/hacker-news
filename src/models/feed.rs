use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FeedType {
    Top,
    New,
    Best,
    Ask,
    Show,
    Jobs,
}

impl FeedType {
    pub const ALL: [FeedType; 6] = [
        FeedType::Top,
        FeedType::New,
        FeedType::Best,
        FeedType::Ask,
        FeedType::Show,
        FeedType::Jobs,
    ];

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "top" => FeedType::Top,
            "new" => FeedType::New,
            "best" => FeedType::Best,
            "ask" => FeedType::Ask,
            "show" => FeedType::Show,
            "jobs" | "job" => FeedType::Jobs,
            _ => FeedType::Top,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::New => "new",
            Self::Best => "best",
            Self::Ask => "ask",
            Self::Show => "show",
            Self::Jobs => "jobs",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Top => "Top Stories",
            Self::New => "Newest",
            Self::Best => "Best",
            Self::Ask => "Ask HN",
            Self::Show => "Show HN",
            Self::Jobs => "Jobs",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Top => "🔥",
            Self::New => "⚡",
            Self::Best => "🏆",
            Self::Ask => "💬",
            Self::Show => "💡",
            Self::Jobs => "💼",
        }
    }
}

impl std::fmt::Display for FeedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
