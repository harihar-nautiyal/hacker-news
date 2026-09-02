use actix_web::{App, test, web::Data};
use maud_htmx::AppState;
use maud_htmx::models::{Comment, FeedType, Story, StorySummary};
use maud_htmx::routes::{
    feed::{get_feed, get_feed_page},
    index::index,
    item::get_item_detail,
    search::search_stories,
};
use maud_htmx::storage::DbStore;
use std::sync::Arc;
use tempfile::tempdir;

fn sample_story(id: i64) -> Story {
    Story::builder()
        .id(id)
        .title(format!("Show HN: High-performance Rust App #{}", id))
        .url(Some("https://example.com/app".to_string()))
        .domain(Some("example.com".to_string()))
        .author("ferris".to_string())
        .points(120)
        .time_ago("1h ago".to_string())
        .text(Some("Welcome to our test story".to_string()))
        .num_comments(1)
        .comments(vec![Comment::builder()
            .id(901)
            .author("tester".to_string())
            .text("Nice test!".to_string())
            .time_ago("30m ago".to_string())
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
            .id("101".to_string())
            .rank(1)
            .title("Test Story 101".to_string())
            .url(Some("https://example.com/101".to_string()))
            .domain(Some("example.com".to_string()))
            .author("alice".to_string())
            .points(100)
            .num_comments(10)
            .time_ago("2h ago".to_string())
            .has_text(false)
            .is_external(true)
            .build(),
    ]
}

#[actix_web::test]
async fn test_index_route_with_db() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("routes_test.redb");
    let store = DbStore::open(&db_path).expect("open failed");

    // Pre-populate top feed in database
    store
        .put_feed(FeedType::Top, 0, &sample_summaries())
        .unwrap();

    let state = Arc::new(AppState::new(Arc::new(store)));
    let app = test::init_service(
        App::new()
            .app_data(Data::from(state.clone()))
            .service(index)
            .service(get_feed)
            .service(get_feed_page)
            .service(search_stories)
            .service(get_item_detail),
    )
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("Test Story 101"));
    assert!(body_str.contains("<!DOCTYPE html>"));
}

#[actix_web::test]
async fn test_feed_partial_route() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("routes_feed.redb");
    let store = DbStore::open(&db_path).expect("open failed");

    store
        .put_feed(FeedType::New, 0, &sample_summaries())
        .unwrap();

    let state = Arc::new(AppState::new(Arc::new(store)));
    let app = test::init_service(
        App::new()
            .app_data(Data::from(state.clone()))
            .service(get_feed),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/feed?type=new")
        .insert_header(("hx-request", "true"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("Test Story 101"));
}

#[actix_web::test]
async fn test_item_detail_routes() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("routes_item.redb");
    let store = DbStore::open(&db_path).expect("open failed");

    // Pre-populate story 101 and top feed
    store.put_story(&sample_story(101)).unwrap();
    store
        .put_feed(FeedType::Top, 0, &sample_summaries())
        .unwrap();

    let state = Arc::new(AppState::new(Arc::new(store)));
    let app = test::init_service(
        App::new()
            .app_data(Data::from(state.clone()))
            .service(get_item_detail),
    )
    .await;

    // 1. HTMX request returns partial
    let htmx_req = test::TestRequest::get()
        .uri("/item/101")
        .insert_header(("hx-request", "true"))
        .to_request();
    let htmx_resp = test::call_service(&app, htmx_req).await;
    assert!(htmx_resp.status().is_success());
    let htmx_body = String::from_utf8_lossy(&test::read_body(htmx_resp).await).to_string();
    assert!(htmx_body.contains("Show HN: High-performance Rust App #101"));
    assert!(htmx_body.contains("Nice test!"));
    // HTMX partial does not contain <!DOCTYPE html>
    assert!(!htmx_body.contains("<!DOCTYPE html>"));

    // 2. Direct browser navigation returns full page
    let full_req = test::TestRequest::get().uri("/item/101").to_request();
    let full_resp = test::call_service(&app, full_req).await;
    assert!(full_resp.status().is_success());
    let full_body = String::from_utf8_lossy(&test::read_body(full_resp).await).to_string();
    assert!(full_body.contains("<!DOCTYPE html>"));
    assert!(full_body.contains("Show HN: High-performance Rust App #101"));
}

#[actix_web::test]
async fn test_search_route_with_db() {
    let dir = tempdir().expect("tempdir failed");
    let db_path = dir.path().join("routes_search.redb");
    let store = DbStore::open(&db_path).expect("open failed");

    // Pre-populate search query in database
    store
        .put_search("actix", 0, &sample_summaries())
        .unwrap();

    let state = Arc::new(AppState::new(Arc::new(store)));
    let app = test::init_service(
        App::new()
            .app_data(Data::from(state.clone()))
            .service(search_stories),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/search?q=actix")
        .insert_header(("hx-request", "true"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("Test Story 101"));
    assert!(body_str.contains("Results for"));
    assert!(body_str.contains("actix"));
}
