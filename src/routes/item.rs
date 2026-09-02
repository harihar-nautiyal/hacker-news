use actix_web::{
    HttpRequest, HttpResponse, Responder, get,
    web::{Data, Path, Query},
};
use maud::{Markup, html};
use serde::Deserialize;

use crate::AppState;
use crate::components::comments_section::comments_section;
use crate::components::story_detail::story_detail_lazy;
use crate::models::FeedType;
use crate::routes::index::Index;
use crate::utils::feed::Feed;

#[derive(Debug, Clone, Deserialize)]
pub struct ItemQuery {
    pub refresh: Option<bool>,
}

fn story_not_found(err: impl std::fmt::Display) -> Markup {
    html! {
        div class="p-8 text-center text-red-400" {
            h3 class="text-lg font-bold mb-2" { "Story Not Found" }
            p class="text-sm" { (err) }
        }
    }
}

// Story item detail endpoint (supports both HTMX partial with lazy comments and direct full-page load)
#[get("/item/{id}")]
pub async fn get_item_detail(
    req: HttpRequest,
    service: Data<AppState>,
    path: Path<i64>,
    query: Query<ItemQuery>,
) -> impl Responder {
    let id = path.into_inner();
    let force_refresh = query.refresh.unwrap_or(false);

    let detail = match service.get_item(id, force_refresh).await {
        Ok(d) => d,
        Err(err) => {
            return HttpResponse::NotFound()
                .content_type("text/html; charset=utf-8")
                .body(story_not_found(&err).into_string());
        }
    };

    let is_htmx = req.headers().contains_key("hx-request");

    if is_htmx {
        // HTMX SPA navigation: Render header instantly with lazy comments placeholder
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .insert_header(("Cache-Control", "private, max-age=300, stale-while-revalidate=60"))
            .body(story_detail_lazy(&detail).into_string())
    } else {
        // Direct browser visit / Search Engine Crawler: render full page with complete comments (100% SEO)
        let stories = service
            .get_feed(FeedType::Top, 0, false)
            .await
            .unwrap_or_default();

        let page_title = format!("{} | Hacker News", detail.title);

        let index_page = Index::builder()
            .title(page_title)
            .active_feed(FeedType::Top)
            .has_active_detail(true)
            .category_icon("🔥".to_string())
            .category_title("Top Stories".to_string())
            .feed_type(FeedType::Top)
            .search_query(String::new())
            .stories(stories)
            .next_page(1)
            .active_detail(Some(detail))
            .build();

        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .insert_header(("Cache-Control", "public, max-age=120, stale-while-revalidate=60"))
            .body(index_page.render().into_string())
    }
}

// Background endpoint for lazy-loading comments
#[get("/item/{id}/comments")]
pub async fn get_item_comments(
    service: Data<AppState>,
    path: Path<i64>,
    query: Query<ItemQuery>,
) -> impl Responder {
    let id = path.into_inner();
    let force_refresh = query.refresh.unwrap_or(false);

    match service.get_item(id, force_refresh).await {
        Ok(detail) => HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .insert_header(("Cache-Control", "private, max-age=300, stale-while-revalidate=60"))
            .body(comments_section(&detail).into_string()),
        Err(err) => HttpResponse::InternalServerError()
            .content_type("text/html; charset=utf-8")
            .body(format!(
                "<div id=\"comments-container\" class=\"p-6 text-center text-red-400\"><p class=\"text-sm\">Failed to load comments: {}</p></div>",
                err
            )),
    }
}
