use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgoliaHit {
    #[serde(default, rename = "objectID")]
    pub object_id: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub points: Option<i64>,
    pub num_comments: Option<i64>,
    pub created_at_i: Option<i64>,
    pub story_text: Option<String>,
    pub comment_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgoliaSearchResponse {
    #[serde(default)]
    pub hits: Vec<AlgoliaHit>,
    pub page: Option<u32>,
    #[serde(rename = "nbPages")]
    pub nb_pages: Option<u32>,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgoliaComment {
    pub id: i64,
    pub author: Option<String>,
    pub text: Option<String>,
    pub created_at_i: Option<i64>,
    pub parent_id: Option<i64>,
    #[serde(default)]
    pub children: Vec<AlgoliaComment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgoliaItemResponse {
    pub id: i64,
    pub title: Option<String>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub points: Option<i64>,
    pub created_at_i: Option<i64>,
    pub text: Option<String>,
    #[serde(default)]
    pub children: Vec<AlgoliaComment>,
}
