pub mod components;
pub mod models;
pub mod routes;
pub mod storage;
pub mod utils;

use crate::models::{Story, StorySummary};
use crate::storage::DbStore;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub const FEED_CACHE_TTL_SECS: u64 = 300; // 5 minutes
pub const SEARCH_CACHE_TTL_SECS: u64 = 300; // 5 minutes
pub const ITEM_CACHE_TTL_SECS: u64 = 1800; // 30 minutes

#[derive(Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub expires_at: Instant,
}

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub store: Arc<DbStore>,
    pub feed_cache: DashMap<String, CacheEntry<Vec<StorySummary>>>,
    pub item_cache: DashMap<i64, CacheEntry<Story>>,
}

impl AppState {
    pub fn new(store: Arc<DbStore>) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .user_agent("Mozilla/5.0 (compatible; HackerNewsSPA/1.0; +https://news.ycombinator.com)")
                .build()
                .unwrap_or_default(),
            store,
            feed_cache: DashMap::new(),
            item_cache: DashMap::new(),
        }
    }

    pub fn with_db_path(path: &str) -> anyhow::Result<Self> {
        let store = DbStore::open(path)?;
        Ok(Self::new(Arc::new(store)))
    }
}
