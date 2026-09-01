use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    middleware::Compress,
    web::{self, Data},
};
use maud_htmx::{
    AppState,
    routes::{
        feed::{get_feed, get_feed_page},
        index::index,
        item::get_item_detail,
        search::search_stories,
    },
};
use rust_embed::RustEmbed;
use std::io::Result;

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

    println!("=============================================");
    println!("  🚀 Hacker News Maud HTMX SPA Starting     ");
    println!("  Listening on http://{}:{}", host, port);
    println!("=============================================");

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(service.clone())
            .service(serve_assets)
            .service(index)
            .service(get_feed)
            .service(get_feed_page)
            .service(search_stories)
            .service(get_item_detail)
    })
    .bind((host, port))?
    .run()
    .await
}

