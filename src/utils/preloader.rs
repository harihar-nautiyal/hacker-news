use crate::AppState;
use crate::models::FeedType;
use crate::utils::feed::Feed;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tracing::{error, info, warn};

/// Preloads page 0 for all 6 feed types,
/// and gently pre-warms the top stories and comment trees into the persistent database.
pub async fn preload_all_feeds(state: Arc<AppState>) {
    info!("🚀 Starting startup pre-warm for feed categories...");

    let mut warm_story_ids: Vec<i64> = Vec::new();

    for feed_type in FeedType::ALL {
        info!("⏳ Preloading feed: {}...", feed_type.label());
        match state.get_feed(feed_type, 0, false).await {
            Ok(stories) => {
                info!(
                    "✅ Preloaded {} stories for feed: {}",
                    stories.len(),
                    feed_type.label()
                );
                // Pre-warm the top 5 stories for each feed
                for s in stories.into_iter().take(5) {
                    if let Ok(id) = s.id.parse::<i64>() {
                        warm_story_ids.push(id);
                    }
                }
            }
            Err(e) => {
                warn!("⚠️ Failed to preload feed {}: {:#}", feed_type.label(), e);
            }
        }
    }

    warm_story_ids.dedup();
    info!(
        "📥 Preloading full story documents & comment trees for {} hot stories in background...",
        warm_story_ids.len()
    );

    preload_story_documents(state.clone(), warm_story_ids, 2).await;

    info!("✨ Startup pre-warm complete! App is primed for instant loads.");
}

/// Concurrently fetches and stores full story documents into the database,
/// skipping items that are already cached and utilizing a semaphore with polite spacing.
pub async fn preload_story_documents(
    state: Arc<AppState>,
    story_ids: Vec<i64>,
    max_concurrency: usize,
) {
    if story_ids.is_empty() {
        return;
    }

    // Filter out IDs that already exist in persistent storage
    let missing_ids: Vec<i64> = story_ids
        .into_iter()
        .filter(|&id| {
            state
                .store
                .get_story(id)
                .map(|opt| opt.is_none())
                .unwrap_or(true)
        })
        .collect();

    if missing_ids.is_empty() {
        return;
    }

    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let mut handles = Vec::new();

    for id in missing_ids {
        let sem = semaphore.clone();
        let state_clone = state.clone();

        let handle = tokio::spawn(async move {
            let _permit = sem.acquire().await.ok();
            tokio::time::sleep(Duration::from_millis(50)).await;
            match state_clone.get_item(id, false).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Failed to preload story document #{}: {:#}", id, e);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

/// Spawns a polite background task to preload top story documents returned from a search query or pagination.
pub fn spawn_items_preloader(state: Arc<AppState>, story_ids: Vec<i64>) {
    if story_ids.is_empty() {
        return;
    }

    let top_ids: Vec<i64> = story_ids.into_iter().take(5).collect();
    tokio::spawn(async move {
        preload_story_documents(state, top_ids, 2).await;
    });
}

/// Background periodic sync loop that refreshes active feeds and hot stories every few minutes.
pub async fn start_background_sync(state: Arc<AppState>, interval: Duration) {
    loop {
        tokio::time::sleep(interval).await;
        info!("🔄 Running periodic background feed sync...");

        for feed_type in FeedType::ALL {
            match state.get_feed(feed_type, 0, true).await {
                Ok(stories) => {
                    // Pre-warm the top 5 stories of each feed
                    let top_ids: Vec<i64> = stories
                        .iter()
                        .take(5)
                        .filter_map(|s| s.id.parse::<i64>().ok())
                        .collect();

                    let state_clone = state.clone();
                    tokio::spawn(async move {
                        preload_story_documents(state_clone, top_ids, 2).await;
                    });
                }
                Err(e) => {
                    error!("Error during periodic sync of {}: {:#}", feed_type.label(), e);
                }
            }
        }
    }
}
