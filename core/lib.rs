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

fn login_user(query: Form<models::UsersLogin>) -> HttpResponse {
    let query = query.into_inner();
    let models::UsersLogin {  username, password} = &query;
    let mut success = false;
    use schema::users::dsl as n;

    let users: Vec<String> = n::users
        .filter(n::username.eq(username.to_ascii_uppercase()))
        .filter(n::password.eq(password.to_ascii_uppercase()))
        .select(n::username)
        .load::<String>(&pool::connect())
        .unwrap();

    let result = if password.replace(" ", "").is_empty() || username.replace(" ", "").is_empty() {
        "Error: There are empty fields"
    } else if password.contains(" ") || username.contains(" ") {
        "Error: Fields can not have spaces"
    } else if users.len() == 1 {
        success = true;
        "Success"
    } else {
        "Invalid username/password"
    };

    if success {
        HttpResponse::Ok().body("")
    } else {
        HttpResponse::Unauthorized().reason(result).finish()
    }
}

fn new_user(query: Form<models::UsersForm>) -> HttpResponse {
    let query = query.into_inner();
    let models::UsersForm { name, username, password, password_repeat } = &query;
    let mut success: bool = false;
    use schema::users::dsl as n;

    let users: Vec<String> = n::users
        .filter(n::username.eq(username.to_ascii_uppercase()))
        .select(n::username)
        .load::<String>(&pool::connect())
        .unwrap();


    let result = if users.len() != 0 {
        "Error: The user already exists"
    } else if
            name.replace(" ", "") == "" || username.replace(" ", "") == "" ||
            password.replace(" ", "") == "" || password_repeat.replace(" ", "") == ""
    {
        "Error: There are empty fields"
    } else if
        username.contains(" ") || password.contains(" ") || password_repeat.contains(" ")
    {
        "Error: Username/Password fields can not contain spaces :)"
    } else if password != password_repeat {
        "Error: Passwords are not the same"
    } else {
        if diesel::insert_into(n::users)
            .values( models::Users{
                name: name.to_ascii_uppercase().clone(),
                username: username.to_ascii_uppercase().clone(),
                password: password.to_ascii_uppercase().clone(),
            } )
            .execute(&pool::connect())
            .is_ok() {
            success = true;
            "Success"
        } else {
            "Something went wrong, contact Ale :/"
        }
    };

    if success {
        HttpResponse::Ok().body("")
    } else {
        HttpResponse::Unauthorized().reason(result).finish()
    }


}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/myget").route(web::get().to(handle_info_get)))
        .service(web::resource("/mypost").route(web::post().to(handle_info_post)))
        .service(web::resource("/new_user").route(web::post().to(new_user)))
        .service(web::resource("/log_user").route(web::post().to(login_user)));
}
