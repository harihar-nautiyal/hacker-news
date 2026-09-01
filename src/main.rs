use crate::models::{Story, StorySummary};
use crate::{components::ui::button::Button, routes::document::Document};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{self, Data},
};
use dashmap::DashMap;
use maud::html;
use rust_embed::RustEmbed;
use std::io::Result;
use std::time::Duration;
use std::time::Instant;

pub mod components;
pub mod models;
pub mod routes;
pub mod utils;

#[derive(Clone)]
struct CacheEntry<T> {
    pub data: T,
    pub expires_at: Instant,
}

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub feed_cache: DashMap<String, CacheEntry<Vec<StorySummary>>>,
    pub item_cache: DashMap<i64, CacheEntry<Story>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .user_agent("HackerNews-HTMX-SPA/1.0")
                .build()
                .unwrap_or_default(),
            feed_cache: DashMap::new(),
            item_cache: DashMap::new(),
        }
    }
}

#[get("/version")]
async fn version() -> impl Responder {
    let content = html! {
        p { "Hello world" }
        (Button("Click me"))
    };

    Document("Hello world", content)
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
