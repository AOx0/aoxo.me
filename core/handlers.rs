//! GET and POST handlers
//!
//! This module is intended to contain and config within `ServiceRequest` all functions to handle the requests
//!
//! # Examples:
//! ```rust
//!
//! use actix_web::HttpResponse;
//! use actix_web::web::Form;
//!
//! fn handle_info_post(query: Form<models::Info>) -> HttpResponse {
//!     let query = query.into_inner();
//!     let models::Info { name } = &query;
//!
//!
//!     use schema::names::dsl as n;
//!
//!     diesel::insert_into(n::names)
//!         .values( &query )
//!         .execute(&pool::connect())
//!         .unwrap();
//!
//!     HttpResponse::Ok()
//!         .body(format!("Welcome, {}!\nYou were added to the database", name))
//! }
//! ```
//!
//! ```rust
//! use actix_web::HttpResponse;
//!
//! fn handle_info_get() -> HttpResponse {
//!     use schema::names::dsl as n;
//!
//!     let users = n::names
//!         .filter(n::name_id.gt(2))
//!         .select((n::name, n::name_id))
//!         .load::<(String, i64)>(&pool::connect())
//!         .unwrap();
//!
//!
//!     HttpResponse::Ok()
//!         .body(format!("Our users are: {:?}", users))
//! }
//! ```
//!

use actix_web::web::{Form, service, ServiceConfig};
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{HttpResponse, web};
use actix_session::*;
use actix_web::dev::HttpResponseBuilder;
use crate::diesel::prelude::*;
use crate::{models, pool, schema};


trait NoCache {
    fn no_cache(&mut self) -> &mut HttpResponseBuilder;
}

impl NoCache for HttpResponseBuilder {
    fn no_cache(&mut self) -> &mut HttpResponseBuilder {
        self
            .set(CacheControl(vec![CacheDirective::NoCache]))
            .set(CacheControl(vec![CacheDirective::NoStore]));
        self
    }
}


fn login_user(session: Session, query: Form<models::UsersLogin>) -> HttpResponse {
    // println!("En login: {:?}", session.get::<String>("session"));
    let query = query.into_inner();
    let models::UsersLogin {  username, password} = &query;
    use schema::users::dsl as n;

    let users: Vec<String> = n::users
        .filter(n::username.eq(username.to_ascii_uppercase()))
        .filter(n::password.eq(password.to_ascii_uppercase()))
        .select(n::username)
        .load::<String>(&pool::connect())
        .unwrap();

    let cookie: String;

    if password.replace(" ", "").is_empty() || username.replace(" ", "").is_empty() {
        HttpResponse::Unauthorized()
            .no_cache()
            .reason("Error: There are empty fields").finish()
    } else if password.contains(" ") || username.contains(" ") {
        HttpResponse::Unauthorized()
            .no_cache()
            .reason("Error: Fields can not have spaces").finish()
    } else if users.len() == 1 {
        cookie = crate::sessions::generate_new_session_cookie();
        crate::sessions::add_session(cookie.clone());
        crate::sessions::associate(username.clone().to_uppercase(), cookie.clone());

        session.set("session",cookie ).unwrap();

        HttpResponse::Ok()
            .no_cache()
            .body("")
    } else {
        HttpResponse::Unauthorized()
            .no_cache()
            .reason("Invalid username/password").finish()
    }
}

/// Handles `/new_user` POST. If valid name, username and passwords then registers the user in the database.
fn new_user(query: Form<models::UsersForm>) -> HttpResponse {
    let query = query.into_inner();
    let models::UsersForm { name, username, password, password_repeat } = &query;
    let mut success: bool = false;
    use schema::users::dsl as n;
    use schema::missions::dsl as m;

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



            if let Ok(id) = n::users.select(n::id)
                .filter(n::username.eq(username.to_ascii_uppercase()))
                .first::<i64>(&pool::connect()) {

                    diesel::insert_into(m::missions)
                        .values( models::NewMission {
                            user_id: id
                        } )
                        .execute(&pool::connect())
                        .unwrap();

                success = true;
                "Success"
            } else {
                "Something went wrong, contact Ale :/"
            }





        } else {
            "Something went wrong, contact Ale :/"
        }
    };

    if success {
        HttpResponse::Ok()
            .no_cache()
            .body("")

    } else {
        HttpResponse::Unauthorized().reason(result)
            .no_cache()
            .finish()
    }


}

fn handle_file(session: Session, file: Form<models::File>) -> HttpResponse {
    let file = file.into_inner();
    let models::File { file, file_id } = &file;

    println!("\n\nUser: {}", crate::sessions::get_user_name(session.get("session").unwrap().unwrap()).unwrap());
    println!("Session: {}", session.get::<String>("session").unwrap().unwrap());
    println!("File ID: {}", file_id);
    println!("{}\n\n", file);

    HttpResponse::Ok().body("")
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/new_user").route(web::post().to(new_user)))
        .service(web::resource("/log_user").route(web::post().to(login_user)))
        .service(web::resource("/mission").route(web::post().to(handle_file)));
}
