use crate::utils::comment::build_comment_tree;
use crate::utils::preloader::spawn_items_preloader;
use crate::{
    AppState, FEED_CACHE_TTL_SECS, ITEM_CACHE_TTL_SECS, SEARCH_CACHE_TTL_SECS,
    models::{
        AlgoliaComment, AlgoliaItemResponse, AlgoliaSearchResponse, FeedType, FirebaseItem,
        Story, StorySummary,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

#[async_trait]
pub trait Feed {
    async fn get_feed(
        &self,
        feed_type: FeedType,
        page: u32,
        force_refresh: bool,
    ) -> Result<Vec<StorySummary>>;
    async fn search(&self, query: &str, page: u32) -> Result<Vec<StorySummary>>;
    async fn fetch(&self, url: &str, page: u32) -> Result<Vec<StorySummary>>;
    async fn get_item(&self, id: i64, force_refresh: bool) -> Result<Story>;
}

#[async_trait]
impl Feed for AppState {
    async fn get_feed(
        &self,
        feed_type: FeedType,
        page: u32,
        force_refresh: bool,
    ) -> Result<Vec<StorySummary>> {
        let cache_key = format!("{}:{}", feed_type.as_str(), page);

        // 1. Check L1 Memory Cache (RAM)
        if !force_refresh {
            if let Some(entry) = self.feed_cache.get(&cache_key)
                && Instant::now() < entry.expires_at
            {
                debug!("L1 RAM hit for feed: {}", cache_key);
                return Ok(entry.data.clone());
            }
        }

        // 2. Check L2 Persistent Storage (redb Disk)
        if !force_refresh {
            if let Ok(Some(stories)) = self.store.get_feed(feed_type, page, Some(FEED_CACHE_TTL_SECS)) {
                debug!("L2 Disk hit for feed: {}", cache_key);
                // Populate L1 RAM
                self.feed_cache.insert(
                    cache_key.clone(),
                    crate::CacheEntry {
                        data: stories.clone(),
                        expires_at: Instant::now() + Duration::from_secs(FEED_CACHE_TTL_SECS),
                    },
                );

                // Background pre-warm story documents for this feed page
                let story_ids: Vec<i64> = stories
                    .iter()
                    .filter_map(|s| s.id.parse::<i64>().ok())
                    .collect();
                spawn_items_preloader(Arc::new(self.clone()), story_ids);

                return Ok(stories);
            }
        }

        // 3. L3 Network Fetch with automatic Firebase fallback
        let url = match feed_type {
            FeedType::Top => format!(
                "https://hn.algolia.com/api/v1/search?tags=front_page&hitsPerPage=30&page={}",
                page
            ),
            FeedType::New => format!(
                "https://hn.algolia.com/api/v1/search_by_date?tags=story&hitsPerPage=30&page={}",
                page
            ),
            FeedType::Best => format!(
                "https://hn.algolia.com/api/v1/search?numericFilters=points>150&hitsPerPage=30&page={}",
                page
            ),
            FeedType::Ask => format!(
                "https://hn.algolia.com/api/v1/search?tags=ask_hn&hitsPerPage=30&page={}",
                page
            ),
            FeedType::Show => format!(
                "https://hn.algolia.com/api/v1/search?tags=show_hn&hitsPerPage=30&page={}",
                page
            ),
            FeedType::Jobs => format!(
                "https://hn.algolia.com/api/v1/search?tags=job&hitsPerPage=30&page={}",
                page
            ),
        };

        info!("L3 Network fetch for feed: {} (page {})", feed_type.label(), page);
        
        let result = match self.fetch(&url, page).await {
            Ok(stories) => stories,
            Err(e) => {
                warn!(
                    "Algolia fetch failed ({:#}), falling back to official Firebase HN API...",
                    e
                );
                fetch_firebase_feed(&self.client, feed_type, page).await?
            }
        };

        // Persist to L2 Disk
        if let Err(e) = self.store.put_feed(feed_type, page, &result) {
            warn!("Failed to persist feed {} to redb: {:#}", cache_key, e);
        }

        // Store into L1 RAM
        self.feed_cache.insert(
            cache_key,
            crate::CacheEntry {
                data: result.clone(),
                expires_at: Instant::now() + Duration::from_secs(FEED_CACHE_TTL_SECS),
            },
        );

        // Preload all story documents from this feed in the background
        let story_ids: Vec<i64> = result
            .iter()
            .filter_map(|s| s.id.parse::<i64>().ok())
            .collect();
        spawn_items_preloader(Arc::new(self.clone()), story_ids);

        Ok(result)
    }

    async fn search(&self, query: &str, page: u32) -> Result<Vec<StorySummary>> {
        let query = query.trim();

        if query.is_empty() {
            return self.get_feed(FeedType::Top, 0, false).await;
        }

        let cache_key = format!("search:{}:{}", query.to_lowercase(), page);

        // 1. Check L1 Memory Cache (RAM)
        if let Some(entry) = self.feed_cache.get(&cache_key)
            && Instant::now() < entry.expires_at
        {
            debug!("L1 RAM hit for search: {}", cache_key);
            return Ok(entry.data.clone());
        }

        // 2. Check L2 Persistent Storage (redb Disk)
        if let Ok(Some(stories)) = self.store.get_search(query, page, Some(SEARCH_CACHE_TTL_SECS)) {
            debug!("L2 Disk hit for search: {}", cache_key);
            self.feed_cache.insert(
                cache_key.clone(),
                crate::CacheEntry {
                    data: stories.clone(),
                    expires_at: Instant::now() + Duration::from_secs(SEARCH_CACHE_TTL_SECS),
                },
            );

            // Preload story items in background
            let story_ids: Vec<i64> = stories
                .iter()
                .filter_map(|s| s.id.parse::<i64>().ok())
                .collect();
            spawn_items_preloader(Arc::new(self.clone()), story_ids);

            return Ok(stories);
        }

        // 3. L3 Network Search
        let url = format!(
            "https://hn.algolia.com/api/v1/search?query={}&tags=story&hitsPerPage=30&page={}",
            urlencoding::encode(query),
            page
        );

        info!("L3 Network search for: '{}' (page {})", query, page);
        let result = match self.fetch(&url, page).await {
            Ok(stories) => stories,
            Err(e) => {
                warn!("Algolia search failed ({:#}), searching across available feeds...", e);
                let mut candidates = Vec::new();
                for ft in [FeedType::Top, FeedType::New, FeedType::Ask, FeedType::Show, FeedType::Best] {
                    if let Ok(stories) = self.get_feed(ft, 0, false).await {
                        candidates.extend(stories);
                    }
                }
                let q_lower = query.to_lowercase();
                let mut filtered: Vec<StorySummary> = candidates
                    .into_iter()
                    .filter(|s| s.title.to_lowercase().contains(&q_lower) || s.author.to_lowercase().contains(&q_lower))
                    .collect();
                filtered.dedup_by_key(|s| s.id.clone());
                if filtered.is_empty() {
                    filtered = self.get_feed(FeedType::Top, 0, false).await.unwrap_or_default();
                }
                filtered
            }
        };

        // Persist to L2 Disk
        if let Err(e) = self.store.put_search(query, page, &result) {
            warn!("Failed to persist search '{}' to redb: {:#}", query, e);
        }

        // Store into L1 RAM
        self.feed_cache.insert(
            cache_key,
            crate::CacheEntry {
                data: result.clone(),
                expires_at: Instant::now() + Duration::from_secs(SEARCH_CACHE_TTL_SECS),
            },
        );

        // Preload all story documents from this search response in background
        let story_ids: Vec<i64> = result
            .iter()
            .filter_map(|s| s.id.parse::<i64>().ok())
            .collect();
        spawn_items_preloader(Arc::new(self.clone()), story_ids);

        Ok(result)
    }

    async fn fetch(&self, url: &str, page: u32) -> Result<Vec<StorySummary>> {
        let mut attempts = 0;
        let search_res: AlgoliaSearchResponse = loop {
            attempts += 1;
            let response = self.client.get(url).send().await;
            match response {
                Ok(resp) if resp.status().is_success() => {
                    match resp.json::<AlgoliaSearchResponse>().await {
                        Ok(data) => break data,
                        Err(_e) if attempts < 2 => {
                            tokio::time::sleep(Duration::from_millis(150 * attempts)).await;
                            continue;
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                Ok(resp)
                    if attempts < 2
                        && (resp.status().is_server_error()
                            || resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS) =>
                {
                    tokio::time::sleep(Duration::from_millis(200 * attempts)).await;
                    continue;
                }
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    anyhow::bail!("Algolia API error {}: {}", status, text);
                }
                Err(_e) if attempts < 2 => {
                    tokio::time::sleep(Duration::from_millis(150 * attempts)).await;
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        };

        let rank_offset = (page as usize) * 30;
        let summaries: Vec<StorySummary> = search_res
            .hits
            .into_iter()
            .enumerate()
            .filter_map(|(idx, hit)| {
                Some(
                    StorySummary::builder()
                        .id(hit.object_id)
                        .rank(rank_offset + idx + 1)
                        .title(hit.title.unwrap_or_else(|| "Untitled".to_string()))
                        .url(hit.url.clone())
                        .domain(hit.url.as_deref().and_then(extract_domain))
                        .author(hit.author.unwrap_or_else(|| "anonymous".to_string()))
                        .points(hit.points.unwrap_or(0))
                        .num_comments(hit.num_comments.unwrap_or(0))
                        .time_ago(
                            hit.created_at_i
                                .map(format_time_ago)
                                .unwrap_or_else(|| "recently".to_string()),
                        )
                        .has_text(hit.story_text.is_some())
                        .is_external(hit.url.is_some())
                        .build(),
                )
            })
            .collect();

        Ok(summaries)
    }

    async fn get_item(&self, id: i64, force_refresh: bool) -> Result<Story> {
        // 1. Check L1 Memory Cache (RAM)
        if !force_refresh {
            if let Some(cache) = self.item_cache.get(&id)
                && Instant::now() < cache.expires_at
            {
                debug!("L1 RAM hit for story item #{}", id);
                return Ok(cache.data.clone());
            }
        }

        // 2. Check L2 Persistent Storage (redb Disk)
        if !force_refresh {
            if let Ok(Some(story)) = self.store.get_story(id) {
                debug!("L2 Disk hit for story item #{}", id);
                self.item_cache.insert(
                    id,
                    crate::CacheEntry {
                        data: story.clone(),
                        expires_at: Instant::now() + Duration::from_secs(ITEM_CACHE_TTL_SECS),
                    },
                );
                return Ok(story);
            }
        }

        // 3. L3 Network Fetch (Algolia HN API with Firebase fallback)
        let url = format!("https://hn.algolia.com/api/v1/items/{}", id);
        let story_res = self.fetch_algolia_item(&url, id).await;

        let story = match story_res {
            Ok(s) => s,
            Err(e) => {
                warn!("Algolia item fetch failed for #{id} ({:#}), falling back to official Firebase HN API...", e);
                fetch_firebase_item(&self.client, id).await?
            }
        };

        // Persist to L2 Disk
        if let Err(e) = self.store.put_story(&story) {
            warn!("Failed to persist story #{} to redb: {:#}", id, e);
        }

        // Store into L1 RAM
        self.item_cache.insert(
            id,
            crate::CacheEntry {
                data: story.clone(),
                expires_at: Instant::now() + Duration::from_secs(ITEM_CACHE_TTL_SECS),
            },
        );

        Ok(story)
    }
}

impl AppState {
    async fn fetch_algolia_item(&self, url: &str, id: i64) -> Result<Story> {
        let mut attempts = 0;
        let item: AlgoliaItemResponse = loop {
            attempts += 1;
            let response = self.client.get(url).send().await;
            match response {
                Ok(resp) if resp.status().is_success() => {
                    match resp.json::<AlgoliaItemResponse>().await {
                        Ok(data) => break data,
                        Err(_e) if attempts < 2 => {
                            tokio::time::sleep(Duration::from_millis(150 * attempts)).await;
                            continue;
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                Ok(resp)
                    if attempts < 2
                        && (resp.status().is_server_error()
                            || resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS) =>
                {
                    tokio::time::sleep(Duration::from_millis(200 * attempts)).await;
                    continue;
                }
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    anyhow::bail!("Algolia API error {}: {}", status, text);
                }
                Err(_e) if attempts < 2 => {
                    tokio::time::sleep(Duration::from_millis(150 * attempts)).await;
                    continue;
                }
                Err(e) => return Err(e.into()),
            }
        };

        let author = item.author.as_deref().unwrap_or("anonymous");
        let (comments, total_comments) = build_comment_tree(&item.children, author, 0);
        let story: Story = Story::builder()
            .id(id)
            .title(item.title.unwrap_or_else(|| format!("Story #{}", id)))
            .url(item.url.clone())
            .domain(item.url.as_deref().and_then(extract_domain))
            .author(item.author.unwrap_or_else(|| "anonymous".to_string()))
            .points(item.points.unwrap_or(0))
            .time_ago(
                item.created_at_i
                    .map(format_time_ago)
                    .unwrap_or_else(|| "recently".to_string()),
            )
            .text(item.text)
            .num_comments(total_comments)
            .comments(comments)
            .hn_url(format!("https://news.ycombinator.com/item?id={}", id))
            .build();

        Ok(story)
    }
}

pub async fn fetch_firebase_feed(
    client: &reqwest::Client,
    feed_type: FeedType,
    page: u32,
) -> Result<Vec<StorySummary>> {
    let endpoint = match feed_type {
        FeedType::Top => "topstories",
        FeedType::New => "newstories",
        FeedType::Best => "beststories",
        FeedType::Ask => "askstories",
        FeedType::Show => "showstories",
        FeedType::Jobs => "jobstories",
    };
    let url = format!("https://hacker-news.firebaseio.com/v0/{}.json", endpoint);
    let ids: Vec<i64> = client.get(&url).send().await?.json().await?;

    let start = (page as usize) * 30;
    if start >= ids.len() {
        return Ok(Vec::new());
    }
    let end = (start + 30).min(ids.len());
    let page_ids = &ids[start..end];

    let mut set = tokio::task::JoinSet::new();
    for (idx, id) in page_ids.iter().copied().enumerate() {
        let client = client.clone();
        set.spawn(async move {
            let item_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
            let item: Option<FirebaseItem> = client
                .get(&item_url)
                .send()
                .await
                .ok()?
                .json()
                .await
                .ok();
            Some((idx, id, item))
        });
    }

    let mut results: Vec<(usize, i64, Option<FirebaseItem>)> = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(Some(item_tuple)) = res {
            results.push(item_tuple);
        }
    }
    results.sort_by_key(|(idx, _, _)| *idx);

    let rank_offset = start;
    let summaries: Vec<StorySummary> = results
        .into_iter()
        .filter_map(|(idx, id, maybe_item)| {
            let item = maybe_item?;
            if item.deleted.unwrap_or(false) || item.dead.unwrap_or(false) {
                return None;
            }
            Some(
                StorySummary::builder()
                    .id(item.id.to_string())
                    .rank(rank_offset + idx + 1)
                    .title(item.title.unwrap_or_else(|| format!("Story #{}", id)))
                    .url(item.url.clone())
                    .domain(item.url.as_deref().and_then(extract_domain))
                    .author(item.by.unwrap_or_else(|| "anonymous".to_string()))
                    .points(item.score.unwrap_or(0))
                    .num_comments(item.descendants.map(|d| d as i64).unwrap_or(0))
                    .time_ago(
                        item.time
                            .map(format_time_ago)
                            .unwrap_or_else(|| "recently".to_string()),
                    )
                    .has_text(item.text.is_some())
                    .is_external(item.url.is_some())
                    .build(),
            )
        })
        .collect();

    Ok(summaries)
}

pub async fn fetch_firebase_item(client: &reqwest::Client, id: i64) -> Result<Story> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let item: FirebaseItem = client.get(&url).send().await?.json().await?;

    let author = item.by.unwrap_or_else(|| "anonymous".to_string());
    let mut comments_raw = Vec::new();

    if let Some(kids) = item.kids {
        let mut set = tokio::task::JoinSet::new();
        for kid_id in kids.into_iter().take(30) {
            let c = client.clone();
            set.spawn(async move {
                fetch_firebase_comment(&c, kid_id, 0, 4).await
            });
        }
        while let Some(res) = set.join_next().await {
            if let Ok(Some(c)) = res {
                comments_raw.push(c);
            }
        }
    }

    let (comments, total_comments) = build_comment_tree(&comments_raw, &author, 0);
    let total_comments = item.descendants.unwrap_or(total_comments);

    let story = Story::builder()
        .id(id)
        .title(item.title.unwrap_or_else(|| format!("Story #{}", id)))
        .url(item.url.clone())
        .domain(item.url.as_deref().and_then(extract_domain))
        .author(author)
        .points(item.score.unwrap_or(0))
        .time_ago(
            item.time
                .map(format_time_ago)
                .unwrap_or_else(|| "recently".to_string()),
        )
        .text(item.text)
        .num_comments(total_comments)
        .comments(comments)
        .hn_url(format!("https://news.ycombinator.com/item?id={}", id))
        .build();

    Ok(story)
}

fn fetch_firebase_comment<'a>(
    client: &'a reqwest::Client,
    id: i64,
    depth: usize,
    max_depth: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<AlgoliaComment>> + Send + 'a>> {
    Box::pin(async move {
        if depth > max_depth {
            return None;
        }
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
        let item: FirebaseItem = client.get(&url).send().await.ok()?.json().await.ok()?;
        if item.deleted.unwrap_or(false) || item.dead.unwrap_or(false) {
            return None;
        }

        let mut children = Vec::new();
        if depth + 1 <= max_depth && let Some(kids) = item.kids {
            let mut set = tokio::task::JoinSet::new();
            for kid_id in kids.into_iter().take(8) {
                let c = client.clone();
                set.spawn(async move {
                    fetch_firebase_comment(&c, kid_id, depth + 1, max_depth).await
                });
            }
            while let Some(res) = set.join_next().await {
                if let Ok(Some(child)) = res {
                    children.push(child);
                }
            }
        }

        Some(AlgoliaComment {
            id: item.id,
            author: item.by,
            text: item.text,
            created_at_i: item.time,
            parent_id: item.parent,
            children,
        })
    })
}

pub fn extract_domain(url_str: &str) -> Option<String> {
    if let Ok(parsed) = url::Url::parse(url_str)
        && let Some(host) = parsed.host_str()
    {
        let clean_host = host.strip_prefix("www.").unwrap_or(host);
        return Some(clean_host.to_string());
    }
    None
}

pub fn format_time_ago(unix_timestamp: i64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let diff = now - unix_timestamp;
    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        let mins = diff / 60;
        format!("{}m ago", mins)
    } else if diff < 86400 {
        let hours = diff / 3600;
        format!("{}h ago", hours)
    } else if diff < 2592000 {
        let days = diff / 86400;
        format!("{}d ago", days)
    } else if diff < 31536000 {
        let months = diff / 2592000;
        format!("{}mo ago", months)
    } else {
        let years = diff / 31536000;
        format!("{}y ago", years)
    }
}
