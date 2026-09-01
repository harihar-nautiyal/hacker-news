use actix_web::{
    Responder, get,
    web::{Data, Query},
};
use maud::{Markup, html};
use typed_builder::TypedBuilder;

use crate::AppState;
use crate::components::{
    load_more::load_more,
    stories_items::stories_items,
    stories_list::StoriesList,
};
use crate::models::{FeedType, StorySummary};
use crate::routes::index::FeedQuery;
use crate::utils::feed::Feed;

#[derive(Debug, Clone, TypedBuilder)]
pub struct FeedPage {
    pub feed_type: FeedType,
    pub search_query: String,
    pub stories: Vec<StorySummary>,
    pub next_page: u32,
}

impl FeedPage {
    pub fn render(&self) -> Markup {
        html! {
            (stories_items(&self.stories))
            @if !self.stories.is_empty() {
                (load_more(self.feed_type, self.next_page, &self.search_query))
            }
        }
    }
}

// Feed partial (switches categories or refreshes stories)
#[get("/feed")]
pub async fn get_feed(service: Data<AppState>, query: Query<FeedQuery>) -> impl Responder {
    let search_q = query.query.clone().unwrap_or_default();
    let feed_kind = FeedType::from_str(query.feed_type.as_deref().unwrap_or("top"));

    let stories = if !search_q.trim().is_empty() {
        service.search(&search_q, 0).await.unwrap_or_default()
    } else {
        service
            .get_feed(feed_kind, 0, query.refresh.unwrap_or(false))
            .await
            .unwrap_or_default()
    };

    let (category_icon, category_title) = if search_q.trim().is_empty() {
        (feed_kind.icon().to_string(), feed_kind.label().to_string())
    } else {
        ("🔍".to_string(), format!("Search: {}", search_q.trim()))
    };

    StoriesList::builder()
        .category_icon(&category_icon)
        .category_title(&category_title)
        .stories(&stories)
        .search_query(&search_q)
        .feed_type(feed_kind)
        .next_page(1)
        .build()
        .render()
}

// Pagination / Load more route
#[get("/feed/page")]
pub async fn get_feed_page(service: Data<AppState>, query: Query<FeedQuery>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let search_q = query.query.clone().unwrap_or_default();
    let feed_kind = FeedType::from_str(query.feed_type.as_deref().unwrap_or("top"));

    let stories = if !search_q.trim().is_empty() {
        service.search(&search_q, page).await.unwrap_or_default()
    } else {
        service
            .get_feed(feed_kind, page, false)
            .await
            .unwrap_or_default()
    };

    FeedPage::builder()
        .feed_type(feed_kind)
        .search_query(search_q)
        .stories(stories)
        .next_page(page + 1)
        .build()
        .render()
}
