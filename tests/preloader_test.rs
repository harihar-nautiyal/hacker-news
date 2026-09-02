use maud_htmx::AppState;
use maud_htmx::models::Story;
use maud_htmx::storage::DbStore;
use maud_htmx::utils::preloader::preload_story_documents;
use std::sync::Arc;
use tempfile::tempdir;

fn sample_story(id: i64) -> Story {
    Story::builder()
        .id(id)
        .title(format!("Preload Item #{}", id))
        .url(None)
        .domain(None)
        .author("bot".to_string())
        .points(50)
        .time_ago("1h ago".to_string())
        .text(None)
        .num_comments(0)
        .comments(vec![])
        .hn_url(format!("https://news.ycombinator.com/item?id={}", id))
        .build()
}

#[tokio::test]
async fn test_preload_skips_existing_items() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("preload_test.redb");
    let store = DbStore::open(&db_path).expect("open failed");

    // Pre-insert story 7001
    store.put_story(&sample_story(7001)).unwrap();
    assert_eq!(store.count_stories().unwrap(), 1);

    let state = Arc::new(AppState::new(Arc::new(store)));

    // Request preloading of [7001]
    // Because 7001 is already in DB, preload_story_documents should return immediately without making any network requests
    preload_story_documents(state.clone(), vec![7001], 2).await;

    assert_eq!(state.store.count_stories().unwrap(), 1);
}
