use maud_htmx::{AppState, models::FeedType, utils::feed::Feed};

#[tokio::test]
pub async fn fetch_feed() {
    let state = AppState::new();

    let items = state.get_feed(FeedType::Top, 0, true).await;

    assert!(items.is_ok());
}

#[tokio::test]
pub async fn fetch_item() {
    let state = AppState::new();
    let items = state
        .get_feed(FeedType::Top, 0, true)
        .await
        .expect("Failed to fetch feed");

    let item_id = items[0].clone().id;
    let item_id: i64 = item_id.parse().expect("Failed to parse item id");

    let item = state.get_item(item_id, true).await;

    assert!(item.is_ok());
}

#[tokio::test]
pub async fn fetch_search() {
    let state = AppState::new();
    let result = state.search("Htmx", 0).await.expect("Failed to search");

    assert!(result.len() > 0);
}
