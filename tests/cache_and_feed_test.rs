use maud_htmx::AppState;
use maud_htmx::models::{Comment, FeedType, Story, StorySummary};
use maud_htmx::storage::DbStore;
use maud_htmx::utils::feed::Feed;
use std::sync::Arc;
use tempfile::tempdir;

fn sample_story(id: i64) -> Story {
    Story::builder()
        .id(id)
        .title(format!("Rust Embedded DB with redb #{}", id))
        .url(Some("https://example.com/redb".to_string()))
        .domain(Some("example.com".to_string()))
        .author("rustacean".to_string())
        .points(150)
        .time_ago("3h ago".to_string())
        .text(None)
        .num_comments(1)
        .comments(vec![Comment::builder()
            .id(2001)
            .author("alice".to_string())
            .text("Very snappy".to_string())
            .time_ago("2h ago".to_string())
            .is_op(false)
            .depth(0)
            .total_replies(0)
            .children(vec![])
            .build()])
        .hn_url(format!("https://news.ycombinator.com/item?id={}", id))
        .build()
}

fn sample_summaries() -> Vec<StorySummary> {
    vec![
        StorySummary::builder()
            .id("5001".to_string())
            .rank(1)
            .title("Fastest Hacker News Clone".to_string())
            .url(Some("https://example.com/hn".to_string()))
            .domain(Some("example.com".to_string()))
            .author("speedy".to_string())
            .points(300)
            .num_comments(45)
            .time_ago("1h ago".to_string())
            .has_text(false)
            .is_external(true)
            .build(),
    ]
}

#[tokio::test]
async fn test_multi_tier_story_cache() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("cache_story.redb");

    let store = DbStore::open(&db_path).expect("open failed");
    // Preload story into L2 disk storage
    let story = sample_story(5001);
    store.put_story(&story).expect("put_story failed");

    let state = AppState::new(Arc::new(store));

    // L1 RAM cache is initially empty
    assert!(state.item_cache.get(&5001).is_none());

    // Calling get_item should hit L2 Disk, populate L1 RAM, and return without network call
    let retrieved = state.get_item(5001, false).await.expect("get_item failed");
    assert_eq!(retrieved.id, 5001);
    assert_eq!(retrieved.title, "Rust Embedded DB with redb #5001");

    // Verify L1 RAM is now populated
    assert!(state.item_cache.get(&5001).is_some());
    let ram_cached = state.item_cache.get(&5001).unwrap();
    assert_eq!(ram_cached.data.id, 5001);
}

#[tokio::test]
async fn test_multi_tier_feed_cache() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("cache_feed.redb");

    let store = DbStore::open(&db_path).expect("open failed");
    // Preload feed into L2 disk storage
    let summaries = sample_summaries();
    store
        .put_feed(FeedType::Top, 0, &summaries)
        .expect("put_feed failed");

    let state = AppState::new(Arc::new(store));

    // L1 RAM is empty initially
    let cache_key = format!("{}:{}", FeedType::Top.as_str(), 0);
    assert!(state.feed_cache.get(&cache_key).is_none());

    // Calling get_feed should hit L2 Disk, populate L1 RAM, and return instantly
    let feed_items = state
        .get_feed(FeedType::Top, 0, false)
        .await
        .expect("get_feed failed");
    assert_eq!(feed_items.len(), 1);
    assert_eq!(feed_items[0].id, "5001");
    assert_eq!(feed_items[0].title, "Fastest Hacker News Clone");

    // Verify L1 RAM is now populated
    assert!(state.feed_cache.get(&cache_key).is_some());
}

#[tokio::test]
async fn test_multi_tier_search_cache() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("cache_search.redb");

    let store = DbStore::open(&db_path).expect("open failed");
    // Preload search query results into L2 disk storage
    let summaries = sample_summaries();
    store
        .put_search("instant", 0, &summaries)
        .expect("put_search failed");

    let state = AppState::new(Arc::new(store));

    // Calling search should hit L2 Disk
    let search_items = state.search("instant", 0).await.expect("search failed");
    assert_eq!(search_items.len(), 1);
    assert_eq!(search_items[0].id, "5001");

    let cache_key = "search:instant:0".to_string();
    assert!(state.feed_cache.get(&cache_key).is_some());
}
