use actix_web::{App, HttpServer, Responder, get, web::Data};
use dashmap::DashMap;
use std::io::Result;

pub mod components;
pub mod models;
pub mod routes;

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub feed_cache: DashMap<String, String>,
    pub item_cache: DashMap<i64, String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            feed_cache: DashMap::new(),
            item_cache: DashMap::new(),
        }
    }
}

#[get("/version")]
async fn version() -> impl Responder {
    "0.1.0"
}

#[actix_web::main]
async fn main() -> Result<()> {
    let service = Data::new(AppState::new());
    let host = "127.0.0.1";
    let port = 8080;

    HttpServer::new(move || App::new().app_data(service.clone()).service(version))
        .bind((host, port))?
        .run()
        .await
}
