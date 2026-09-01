use actix_web::{
    Responder, get,
    web::{Data, Query},
};
use serde::Deserialize;

use crate::AppState;
use crate::components::stories_list::StoriesList;
use crate::models::FeedType;
use crate::utils::feed::Feed;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub page: Option<u32>,
}

// Live search endpoint
#[get("/search")]
pub async fn search_stories(service: Data<AppState>, query: Query<SearchQuery>) -> impl Responder {
    let q = query.q.clone().unwrap_or_default();
    let stories = service.search(&q, 0).await.unwrap_or_default();

    let (category_icon, category_title) = if q.trim().is_empty() {
        ("🔥".to_string(), "Top Stories".to_string())
    } else {
        ("🔍".to_string(), format!("Results for \"{}\"", q.trim()))
    };

    StoriesList::builder()
        .category_icon(&category_icon)
        .category_title(&category_title)
        .stories(&stories)
        .search_query(&q)
        .feed_type(FeedType::Top)
        .next_page(1)
        .build()
        .render()
}
