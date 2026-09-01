use crate::utils::comment::build_comment_tree;
use crate::{
    AppState,
    models::{AlgoliaItemResponse, AlgoliaSearchResponse, FeedType, Story, StorySummary},
};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::to_string;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

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
        let feed_type_str: String = to_string(&feed_type)?;
        let cache_key = format!("{}:{}", feed_type_str, page);

        if !force_refresh {
            if let Some(entry) = self.feed_cache.get(&cache_key)
                && Instant::now() < entry.expires_at
            {
                return Ok(entry.data.clone());
            }
        }

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

        let result = self.fetch(&url, page).await?;

        {
            self.feed_cache.insert(
                url.clone(),
                crate::CacheEntry {
                    data: result.clone(),
                    expires_at: Instant::now() + std::time::Duration::from_secs(480),
                },
            );
        }

        Ok(result)
    }

    async fn search(&self, query: &str, page: u32) -> Result<Vec<StorySummary>> {
        let query = query.trim();

        if query.is_empty() {
            return self.get_feed(FeedType::Top, 0, false).await;
        }

        let url = format!(
            "https://hn.algolia.com/api/v1/search?query={}&tags=story&hitsPerPage=30&page={}",
            urlencoding::encode(query),
            page
        );

        self.fetch(&url, page).await
    }

    async fn fetch(&self, url: &str, page: u32) -> Result<Vec<StorySummary>> {
        let response = self.client.get(url).send().await?;
        let search_res: AlgoliaSearchResponse = response.json().await?;
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
        if !force_refresh {
            if let Some(cache) = self.item_cache.get(&id)
                && Instant::now() < cache.expires_at
            {
                return Ok(cache.data.clone());
            }
        }

        let url = format!("https://hn.algolia.com/api/v1/items/{}", id);
        let response = self.client.get(&url).send().await?;
        let item: AlgoliaItemResponse = response.json().await?;
        let author = item.author.as_deref().unwrap_or("anonymous");
        let (comments, total_comments) = build_comment_tree(&item.children, &author, 0);
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

        {
            self.item_cache.insert(
                id,
                crate::CacheEntry {
                    data: story.clone(),
                    expires_at: Instant::now() + std::time::Duration::from_secs(480),
                },
            );
        }

        Ok(story)
    }
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
