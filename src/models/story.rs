use crate::models::Comment;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct Story {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub author: String,
    pub points: i64,
    pub time_ago: String,
    pub text: Option<String>,
    pub num_comments: usize,
    pub comments: Vec<Comment>,
    pub hn_url: String,
}

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct StorySummary {
    pub id: String,
    pub rank: usize,
    pub title: String,
    pub url: Option<String>,
    pub domain: Option<String>,
    pub author: String,
    pub points: i64,
    pub num_comments: i64,
    pub time_ago: String,
    pub has_text: bool,
    pub is_external: bool,
}

impl StorySummary {
    pub fn hn_url(&self) -> String {
        format!("https://news.ycombinator.com/item?id={}", self.id)
    }
}
