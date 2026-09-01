pub mod components;
pub mod models;
pub mod routes;
pub mod utils;

use crate::models::{Story, StorySummary};
use dashmap::DashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub expires_at: Instant,
}

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub feed_cache: DashMap<String, CacheEntry<Vec<StorySummary>>>,
    pub item_cache: DashMap<i64, CacheEntry<Story>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent("HackerNews-HTMX-SPA/1.0")
                .build()
                .unwrap_or_default(),
            feed_cache: DashMap::new(),
            item_cache: DashMap::new(),
        }
    }
}
