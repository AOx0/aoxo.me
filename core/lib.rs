#[macro_use] extern crate diesel;

use actix_web::web::Form;
use actix_web::{HttpResponse, web};
mod models;
mod pool;
mod schema;

pub use pool::init_pool;
use crate::diesel::prelude::*;

fn handle_info_post(query: Form<models::Info>) -> HttpResponse {
    let query = query.into_inner();
    let models::Info { name } = &query;

    use schema::names::dsl as n;

    diesel::insert_into(n::names)
        .values( &query )
        .execute(&pool::connect())
        .unwrap();

    HttpResponse::Ok()
        .body(format!("Welcome, {}!\nYou were added to the database", name))
}

fn handle_info_get() -> HttpResponse {
    use schema::names::dsl as n;

    let users = n::names
        .filter(n::name_id.gt(2))
        .select((n::name, n::name_id))
        .load::<(String, i64)>(&pool::connect())
        .unwrap();


    HttpResponse::Ok()
        .body(format!("Our users are: {:?}", users))
}

fn new_user(query: Form<models::Users>) -> HttpResponse {
    let query = query.into_inner();
    let models::Users { username, password } = &query;

    HttpResponse::Ok()
        .body(format!("Setting user {} with password {}", username, password))

}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/myget").route(web::get().to(handle_info_get)))
        .service(web::resource("/mypost").route(web::post().to(handle_info_post)))
        .service(web::resource("/new_user").route(web::post().to(new_user)));
}
