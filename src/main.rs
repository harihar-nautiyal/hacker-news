use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{self, Data},
};
use maud::html;
use maud_htmx::{AppState, components::ui::button::Button, routes::document::Document};
use rust_embed::RustEmbed;
use std::io::Result;

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
