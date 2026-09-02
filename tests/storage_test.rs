use maud_htmx::models::{Comment, FeedType, Story, StorySummary};
use maud_htmx::storage::DbStore;
use tempfile::tempdir;

fn sample_story(id: i64) -> Story {
    Story::builder()
        .id(id)
        .title(format!("Show HN: High-performance Rust Web App #{}", id))
        .url(Some("https://example.com/rust-app".to_string()))
        .domain(Some("example.com".to_string()))
        .author("ferris".to_string())
        .points(256)
        .time_ago("2h ago".to_string())
        .text(Some("<p>Check out our new Maud + HTMX setup!</p>".to_string()))
        .num_comments(2)
        .comments(vec![
            Comment::builder()
                .id(1001)
                .author("alice".to_string())
                .text("Blazingly fast!".to_string())
                .time_ago("1h ago".to_string())
                .is_op(false)
                .depth(0)
                .total_replies(1)
                .children(vec![Comment::builder()
                    .id(1002)
                    .author("bob".to_string())
                    .text("Agreed, redb is awesome.".to_string())
                    .time_ago("30m ago".to_string())
                    .is_op(false)
                    .depth(1)
                    .total_replies(0)
                    .children(vec![])
                    .build()])
                .build(),
        ])
        .hn_url(format!("https://news.ycombinator.com/item?id={}", id))
        .build()
}

fn sample_summaries(count: usize) -> Vec<StorySummary> {
    (1..=count)
        .map(|i| {
            StorySummary::builder()
                .id(format!("{}", 1000 + i))
                .rank(i)
                .title(format!("Story Item Title #{}", i))
                .url(Some("https://example.com".to_string()))
                .domain(Some("example.com".to_string()))
                .author(format!("author_{}", i))
                .points(10 * i as i64)
                .num_comments(5 * i as i64)
                .time_ago("1h ago".to_string())
                .has_text(false)
                .is_external(true)
                .build()
        })
        .collect()
}

#[test]
fn test_story_crud_and_persistence() {
    let dir = tempdir().expect("Failed to create tempdir");
    let db_path = dir.path().join("test_hn.redb");

    // 1. Create DB and insert story
    {
        let store = DbStore::open(&db_path).expect("Failed to open DB");
        assert_eq!(store.count_stories().unwrap(), 0);

        let story = sample_story(42);
        store.put_story(&story).expect("Failed to put story");

        assert_eq!(store.count_stories().unwrap(), 1);

        let fetched = store.get_story(42).expect("Failed to get story");
        assert!(fetched.is_some());
        let fetched_story = fetched.unwrap();
        assert_eq!(fetched_story.id, 42);
        assert_eq!(fetched_story.title, "Show HN: High-performance Rust Web App #42");
        assert_eq!(fetched_story.comments.len(), 1);
        assert_eq!(fetched_story.comments[0].children.len(), 1);
        assert_eq!(fetched_story.comments[0].children[0].author, "bob");
    }

    // 2. Re-open same DB file to verify persistence across restarts
    {
        let store = DbStore::open(&db_path).expect("Failed to re-open DB");
        assert_eq!(store.count_stories().unwrap(), 1);

        let fetched = store.get_story(42).expect("Failed to get story on reopen");
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().author, "ferris");

        // Non-existent story returns None
        let missing = store.get_story(9999).expect("Querying missing story failed");
        assert!(missing.is_none());
    }
}

#[test]
fn test_batch_story_insertion() {
    let dir = tempdir().expect("Failed to create tempdir");
    let db_path = dir.path().join("test_batch.redb");
    let store = DbStore::open(&db_path).expect("Failed to open DB");

    let stories: Vec<Story> = (100..130).map(sample_story).collect();
    store.put_stories_batch(&stories).expect("Batch insert failed");

    assert_eq!(store.count_stories().unwrap(), 30);

    for s in &stories {
        let fetched = store.get_story(s.id).unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, s.id);
    }
}

#[test]
fn test_feed_caching_and_ttl() {
    let dir = tempdir().expect("Failed to create tempdir");
    let db_path = dir.path().join("test_feed.redb");
    let store = DbStore::open(&db_path).expect("Failed to open DB");

    let summaries = sample_summaries(30);
    store
        .put_feed(FeedType::Top, 0, &summaries)
        .expect("Put feed failed");

    assert_eq!(store.count_feeds().unwrap(), 1);

    // Retrieve with fresh TTL (e.g. 60 secs)
    let cached = store
        .get_feed(FeedType::Top, 0, Some(60))
        .expect("Get feed failed");
    assert!(cached.is_some());
    let cached_stories = cached.unwrap();
    assert_eq!(cached_stories.len(), 30);
    assert_eq!(cached_stories[0].title, "Story Item Title #1");

    // Query another page (page 1) which hasn't been saved yet
    let missing_page = store.get_feed(FeedType::Top, 1, Some(60)).unwrap();
    assert!(missing_page.is_none());

    // Query with max_age_secs = 0 (simulates immediate expiration)
    let expired = store.get_feed(FeedType::Top, 0, Some(0)).unwrap();
    assert!(expired.is_none());
}

#[test]
fn test_search_caching() {
    let dir = tempdir().expect("Failed to create tempdir");
    let db_path = dir.path().join("test_search.redb");
    let store = DbStore::open(&db_path).expect("Failed to open DB");

    let results = sample_summaries(10);
    store
        .put_search("rust web", 0, &results)
        .expect("Put search failed");

    assert_eq!(store.count_searches().unwrap(), 1);

    // Case-insensitive query
    let cached = store
        .get_search("RUST WEB", 0, Some(300))
        .expect("Get search failed");
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().len(), 10);

    // Different query returns None
    let missing = store.get_search("actix", 0, Some(300)).unwrap();
    assert!(missing.is_none());
}
