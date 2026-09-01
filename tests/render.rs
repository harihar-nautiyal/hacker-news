use maud_htmx::{
    components::stories_list::stories_list,
    models::{Comment, FeedType, Story, StorySummary},
    routes::index::Index,
};

#[test]
fn test_index_render_without_detail() {
    let sample_story = StorySummary::builder()
        .id("12345".to_string())
        .rank(1)
        .title("Test Story Title".to_string())
        .url(Some("https://example.com".to_string()))
        .domain(Some("example.com".to_string()))
        .author("alice".to_string())
        .points(42)
        .num_comments(5)
        .time_ago("2h ago".to_string())
        .has_text(false)
        .is_external(true)
        .build();

    let index_page = Index::builder()
        .title("Top | Hacker News SPA".to_string())
        .active_feed(FeedType::Top)
        .has_active_detail(false)
        .category_icon("🔥".to_string())
        .category_title("Top Stories".to_string())
        .feed_type(FeedType::Top)
        .search_query(String::new())
        .stories(vec![sample_story])
        .next_page(1)
        .active_detail(None)
        .build();

    let html = index_page.render().into_string();
    assert!(html.contains("Test Story Title"));
    assert!(html.contains("No Story Selected"));
}

#[test]
fn test_index_render_with_detail() {
    let sample_story = StorySummary::builder()
        .id("12345".to_string())
        .rank(1)
        .title("Test Story Title".to_string())
        .url(Some("https://example.com".to_string()))
        .domain(Some("example.com".to_string()))
        .author("alice".to_string())
        .points(42)
        .num_comments(5)
        .time_ago("2h ago".to_string())
        .has_text(false)
        .is_external(true)
        .build();

    let sample_detail = Story::builder()
        .id(12345)
        .title("Test Story Detail".to_string())
        .url(Some("https://example.com".to_string()))
        .domain(Some("example.com".to_string()))
        .author("alice".to_string())
        .points(42)
        .time_ago("2h ago".to_string())
        .text(Some("<p>Hello world</p>".to_string()))
        .num_comments(1)
        .comments(vec![Comment::builder()
            .id(999)
            .author("bob".to_string())
            .text("Great story!".to_string())
            .time_ago("1h ago".to_string())
            .is_op(false)
            .depth(0)
            .total_replies(0)
            .children(vec![])
            .build()])
        .hn_url("https://news.ycombinator.com/item?id=12345".to_string())
        .build();

    let index_page = Index::builder()
        .title("Test Story Detail | Hacker News".to_string())
        .active_feed(FeedType::Top)
        .has_active_detail(true)
        .category_icon("🔥".to_string())
        .category_title("Top Stories".to_string())
        .feed_type(FeedType::Top)
        .search_query(String::new())
        .stories(vec![sample_story])
        .next_page(1)
        .active_detail(Some(sample_detail))
        .build();

    let html = index_page.render().into_string();
    assert!(html.contains("Test Story Detail"));
    assert!(html.contains("Great story!"));
}

#[test]
fn test_stories_list_render() {
    let sample_story = StorySummary::builder()
        .id("12345".to_string())
        .rank(1)
        .title("Test Story Title".to_string())
        .url(Some("https://example.com".to_string()))
        .domain(Some("example.com".to_string()))
        .author("alice".to_string())
        .points(42)
        .num_comments(5)
        .time_ago("2h ago".to_string())
        .has_text(false)
        .is_external(true)
        .build();

    let html = stories_list(
        "🔥",
        "Top Stories",
        &[sample_story],
        "",
        FeedType::Top,
        1,
    )
    .into_string();

    assert!(html.contains("Test Story Title"));
}

#[test]
fn test_feed_page_render() {
    use maud_htmx::routes::feed::FeedPage;

    let sample_story = StorySummary::builder()
        .id("12345".to_string())
        .rank(1)
        .title("Test Story Title".to_string())
        .url(Some("https://example.com".to_string()))
        .domain(Some("example.com".to_string()))
        .author("alice".to_string())
        .points(42)
        .num_comments(5)
        .time_ago("2h ago".to_string())
        .has_text(false)
        .is_external(true)
        .build();

    let feed_page = FeedPage::builder()
        .feed_type(FeedType::Top)
        .search_query("".to_string())
        .stories(vec![sample_story])
        .next_page(2)
        .build();

    let html = feed_page.render().into_string();
    assert!(html.contains("Test Story Title"));
    assert!(html.contains("Load More Stories"));
}

#[test]
fn test_index_includes_scripts() {
    let index_page = Index::builder()
        .title("Top Stories".to_string())
        .active_feed(FeedType::Top)
        .has_active_detail(false)
        .category_icon("🔥".to_string())
        .category_title("Top Stories".to_string())
        .feed_type(FeedType::Top)
        .search_query("".to_string())
        .stories(vec![])
        .next_page(1)
        .active_detail(None)
        .build();

    let html = index_page.render().into_string();
    assert!(html.contains("/static/scripts/app.js"));
    assert!(html.contains("id=\"detail-progress\""));
}


