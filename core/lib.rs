use actix_web::web::Form;
use actix_web::{HttpResponse, web};
mod models;

fn handle_info_post(query: Form<models::Info>) -> HttpResponse {
    let query = query.into_inner();
    let models::Info { name } = query;

    HttpResponse::Ok()
        .body(format!("Welcome, {}!", name))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mypost").route(web::post().to(handle_info_post)));
}
