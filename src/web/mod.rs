mod web_notes;
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use web_notes::note_service;
use askama::Template;

#[derive(Template)]
#[template(path = "404.html")]
struct ErrorTemplate;

async fn error_400() -> impl Responder {
    HttpResponse::BadRequest().body(ErrorTemplate.render().unwrap())
}
pub async fn start_server() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/notes").configure(note_service))
            .service(Files::new("/static", "./static"))
            .service(web::resource("/404").route(web::get().to(error_400)))
    })
    .bind("127.0.0.1:8080")? // Bind to localhost on port 8080
    .run()
    .await
}

