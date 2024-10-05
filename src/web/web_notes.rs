use actix_web::{web, error, Responder, HttpResponse};
use askama::Template;

use crate::{docted::Docted, notes};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "notes.html")]
struct NoteTemplate{
    notes: Vec<String>
}

#[derive(Deserialize)]
struct AddBody {
    note: String
}
async fn add(info: web::Form<AddBody>) -> impl Responder {
    match notes::exec_notes(crate::cli::NoteAction::Add { content: info.note.clone() }){
       Ok(_) => HttpResponse::Found().append_header(("location", "/notes")).finish(),
       Err(_) =>  HttpResponse::Found().append_header(("location", "/404")).finish(),
    }
}

#[derive(Deserialize)]
struct RemoveBody {
    index: usize
}
async fn remove(info: web::Form<RemoveBody>) -> impl Responder {
    match notes::exec_notes(crate::cli::NoteAction::Remove { id: info.index }){
       Ok(_) => HttpResponse::Found().append_header(("location", "/notes")).finish(),
       Err(_) =>  HttpResponse::Found().append_header(("location", "/404")).finish(),
    }
}

async fn home() -> HttpResponse {
    let docted = match Docted::from_env_dir(){
        Ok(docted) => docted,
        Err(err) => return error::ErrorBadRequest(err).error_response()
    };
    let notes: Vec<String> = docted.notes.entries.iter().map(|f| f.to_string()).collect();
    let template = NoteTemplate {notes: notes.clone()};
    HttpResponse::Ok().body(template.render().unwrap())
}

pub fn note_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(home))
    ).service(
        web::resource("add").route(web::post().to(add))
    ).service(
        web::resource("remove").route(web::post().to(remove))
    );
}
