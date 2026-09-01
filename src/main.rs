use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{self, Data},
};
use maud_htmx::{
    AppState,
    models::FeedType,
    routes::index::Index,
};
use rust_embed::RustEmbed;
use std::io::Result;

#[get("/version")]
async fn version() -> impl Responder {
    Index::builder()
        .title("Hello world".to_string())
        .active_feed(FeedType::Top)
        .has_active_detail(false)
        .category_icon("".to_string())
        .category_title("Top".to_string())
        .feed_type(FeedType::Top)
        .search_query("".to_string())
        .stories(vec![])
        .next_page(1)
        .active_detail(None)
        .build()
        .render()
}

#[derive(RustEmbed)]
#[folder = "static/"]
pub struct Static;

#[get("/static/{filename:.*}")]
async fn serve_assets(path: web::Path<String>) -> impl Responder {
    let filename = path.into_inner();

    match Static::get(&filename) {
        Some(file) => HttpResponse::Ok()
            .content_type(
                mime_guess::from_path(&filename)
                    .first_or_octet_stream()
                    .as_ref(),
            )
            .body(file.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let service = Data::new(AppState::new());
    let host = "127.0.0.1";
    let port = 8080;

    HttpServer::new(move || {
        App::new()
            .app_data(service.clone())
            .service(version)
            .service(serve_assets)
    })
    .bind((host, port))?
    .run()
    .await
}
