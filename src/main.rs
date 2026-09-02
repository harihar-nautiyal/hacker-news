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
        item::{get_item_comments, get_item_detail},
        search::search_stories,
    },
    storage::DbStore,
    utils::preloader::{preload_all_feeds, start_background_sync},
};
use rust_embed::RustEmbed;
use std::io::Result;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

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
            .insert_header(("Cache-Control", "public, max-age=31536000, immutable"))
            .body(file.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[get("/robots.txt")]
async fn robots_txt() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .insert_header(("Cache-Control", "public, max-age=86400"))
        .body("User-agent: *\nAllow: /\nSitemap: /sitemap.xml\n")
}

#[get("/sitemap.xml")]
async fn sitemap_xml() -> impl Responder {
    let sitemap = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>http://localhost:8080/</loc>
    <changefreq>always</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>http://localhost:8080/?type=top</loc>
    <changefreq>hourly</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>http://localhost:8080/?type=new</loc>
    <changefreq>always</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>http://localhost:8080/?type=best</loc>
    <changefreq>daily</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>http://localhost:8080/?type=ask</loc>
    <changefreq>hourly</changefreq>
    <priority>0.7</priority>
  </url>
  <url>
    <loc>http://localhost:8080/?type=show</loc>
    <changefreq>hourly</changefreq>
    <priority>0.7</priority>
  </url>
  <url>
    <loc>http://localhost:8080/?type=jobs</loc>
    <changefreq>daily</changefreq>
    <priority>0.6</priority>
  </url>
</urlset>"#;
    HttpResponse::Ok()
        .content_type("application/xml; charset=utf-8")
        .insert_header(("Cache-Control", "public, max-age=3600"))
        .body(sitemap)
}

#[get("/favicon.ico")]
async fn favicon_ico() -> impl Responder {
    let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><rect width="24" height="24" rx="4" fill="#f59e0b"/><text x="12" y="17" font-family="sans-serif" font-size="15" font-weight="bold" fill="#0a0a0a" text-anchor="middle">Y</text></svg>"##;
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .insert_header(("Cache-Control", "public, max-age=86400, immutable"))
        .body(svg)
}

#[actix_web::main]
async fn main() -> Result<()> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();

    let db_path = std::env::var("HN_DB_PATH").unwrap_or_else(|_| "hn_store.redb".to_string());
    let store = DbStore::open(&db_path).expect("Failed to open persistent redb storage");
    let state = Arc::new(AppState::new(Arc::new(store)));
    let service = Data::from(state.clone());

    let host = "127.0.0.1";
    let port = 8080;

    println!("=============================================");
    println!("  🚀 Hacker News Maud HTMX SPA Starting     ");
    println!("  💾 Persistent DB: {}", db_path);
    println!("  Listening on http://{}:{}", host, port);
    println!("=============================================");

    // Spawn non-blocking background startup pre-warm for all 6 feed categories & story items
    let preload_state = state.clone();
    tokio::spawn(async move {
        preload_all_feeds(preload_state).await;
    });

    // Spawn periodic background sync every 5 minutes
    let sync_state = state.clone();
    tokio::spawn(async move {
        start_background_sync(sync_state, Duration::from_secs(300)).await;
    });

    info!("Starting HTTP server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(service.clone())
            .service(serve_assets)
            .service(robots_txt)
            .service(sitemap_xml)
            .service(favicon_ico)
            .service(index)
            .service(get_feed)
            .service(get_feed_page)
            .service(search_stories)
            .service(get_item_detail)
            .service(get_item_comments)
    })
    .bind((host, port))?
    .run()
    .await
}
