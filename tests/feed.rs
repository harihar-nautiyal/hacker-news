use maud_htmx::{AppState, models::FeedType, storage::DbStore, utils::feed::Feed};
use std::sync::Arc;
use tempfile::tempdir;

fn create_test_state() -> (AppState, tempfile::TempDir) {
    let dir = tempdir().expect("Failed to create tempdir");
    let db_path = dir.path().join("test_feed_integration.redb");
    let store = DbStore::open(&db_path).expect("Failed to open store");
    (AppState::new(Arc::new(store)), dir)
}

#[tokio::test]
pub async fn fetch_feed() {
    let (state, _dir) = create_test_state();

    let items = state.get_feed(FeedType::Top, 0, true).await;

    assert!(items.is_ok());
    let list = items.unwrap();
    assert!(!list.is_empty());

    // Verify it was persisted to redb
    let count = state.store.count_feeds().expect("count_feeds failed");
    assert!(count >= 1);
}

#[tokio::test]
pub async fn fetch_item() {
    let (state, _dir) = create_test_state();
    let items = state
        .get_feed(FeedType::Top, 0, true)
        .await
        .expect("Failed to fetch feed");

    let item_id = items[0].clone().id;
    let item_id: i64 = item_id.parse().expect("Failed to parse item id");

    let item = state.get_item(item_id, true).await;

    assert!(item.is_ok());
    let story = item.unwrap();
    assert_eq!(story.id, item_id);

    // Verify story was persisted to redb
    let cached_story = state.store.get_story(item_id).expect("get_story failed");
    assert!(cached_story.is_some());
}

#[tokio::test]
pub async fn fetch_search() {
    let (state, _dir) = create_test_state();
    let result = state.search("Htmx", 0).await.expect("Failed to search");

    assert!(!result.is_empty());

    // Verify search was persisted to redb
    let count = state.store.count_searches().expect("count_searches failed");
    assert!(count >= 1);
}
