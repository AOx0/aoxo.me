//! GET and POST handlers
//!
//! This module is intended to contain and config within `ServiceRequest` all functions to handle the requests
//!
//!

use actix_web::web::{Form};
use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{http, HttpResponse, web};
use actix_session::*;
use actix_web::dev::HttpResponseBuilder;
use crate::diesel::prelude::*;
use crate::{models, pool, schema, sessions};


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

pub fn get_mission_status(session: Session) {
    use schema::missions::dsl as m;

    let user_id = sessions::get_vital_info(&session).user_id;

    let missions: models::Missions =  m::missions
        .filter(m::user_id.eq(user_id))
        .select((m::mission1, m::mission2, m::mission3, m::mission4, m::mission5))
        .first::<models::Missions>(&pool::connect())
        .unwrap();

    println!("{:?}", missions);

    session.set("mission1", missions.mission1).expect("Failed to get mission1");
    session.set("mission2", missions.mission2).expect("Failed to get mission2");
    session.set("mission3", missions.mission3).expect("Failed to get mission3");
    session.set("mission4", missions.mission4).expect("Failed to get mission4");
    session.set("mission5", missions.mission5).expect("Failed to get mission5");
}


fn login_user(session: Session, query: Form<models::UsersLogin>) -> HttpResponse {
    // println!("En login: {:?}", session.get::<String>("session"));
    let query = query.into_inner();
    let models::UsersLogin {  username, password} = &query;
    use schema::users::dsl as n;

    let username = &username.to_ascii_uppercase();
    let password = &password.to_ascii_uppercase();

    let users: Vec<String> = n::users
        .filter(n::username.eq(username))
        .filter(n::password.eq(password))
        .select(n::username)
        .load::<String>(&pool::connect())
        .unwrap();

    if password.replace(" ", "").is_empty() || username.replace(" ", "").is_empty() {
        HttpResponse::Unauthorized()
            .no_cache()
            .reason("Error: There are empty fields").finish()
    } else if password.contains(" ") || username.contains(" ") {
        HttpResponse::Unauthorized()
            .no_cache()
            .reason("Error: Fields can not have spaces").finish()
    } else if users.len() == 1 {
        let cookie = &crate::sessions::generate_new_session_cookie();

        crate::sessions::add_session(cookie);
        crate::sessions::associate(username, cookie);


        session.set("session",cookie ).unwrap();
        // get_mission_status(session);



        let request = session.get::<String>("goes-to").unwrap().unwrap_or("/".to_string());


        HttpResponse::Ok()
            .no_cache()
            .body(request.to_string())
    } else {
        HttpResponse::Unauthorized()
            .no_cache()
            .reason("Invalid username/password").finish()
    }
}

/// Handles `/new_user` POST. If valid name, username and passwords then registers the user in the database.
fn new_user(query: Form<models::UsersForm>) -> HttpResponse {
    let query = query.into_inner();
    let models::UsersForm { name, username, password, password_repeat,email: mail } = &query;
    let mut success: bool = false;

    use crate::statics::CORREOS;

    use schema::users::dsl as n;
    use schema::missions::dsl as m;

    let username = &username.to_ascii_uppercase();
    let password = &password.to_ascii_uppercase();
    let name = &name.to_ascii_uppercase();

    let users: Vec<String> = n::users
        .filter(n::username.eq(username))
        .select(n::username)
        .load::<String>(&pool::connect())
        .unwrap();

    let emails: Vec<String> = n::users
        .filter(n::email.eq(mail))
        .select(n::email)
        .load::<String>(&pool::connect())
        .unwrap();

    let result = if users.len() != 0 {
        "Error: The user already exists"
    } else if emails.len() != 0 {
        "Error: The email already exists"
    }else if name.replace(" ", "") == "" || username.replace(" ", "") == "" ||
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
                name: name.clone(),
                username: username.clone(),
                password: password.clone(),
                email: mail.clone(),
            } )
            .execute(&pool::connect())
            .is_ok() {


            let user_id = sessions::get_user_id_in_db(&username);

            if user_id != 0 {
                diesel::insert_into(m::missions)
                    .values( models::NewMission { user_id } )
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

    let result =  if !CORREOS.contains(mail){
        success = false;
        "Forbidden"
    } else {
        result
    };

    if success {
        HttpResponse::Ok()
            .no_cache()
            .body("")
    } else {
        HttpResponse::Unauthorized()
            .reason(result)
            .no_cache()
            .finish()
    }


}

fn handle_file(session: Session, file: Form<models::File>) -> HttpResponse {


    if sessions::is_user_logged_in(&session) {
        let session_info = sessions::get_vital_info(&session);
        let file = file.into_inner();
        let models::File {file_id, .. } = &file;


        println!("{:?}", session_info);

        match file_id.as_str() {
            "file1" => println!("file1"),
            "file2" => println!("file2"),
            "file3" => println!("file3"),
            "file4" => println!("file4"),
            "file5" => println!("file5"),
            _ => {}
        }

        HttpResponse::Ok().body("")
    } else {
        HttpResponse::Found()
            .header(http::header::LOCATION, "/login")
            .finish()
            .into_body()
    }
}

fn meet(_: Session) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, "https://up-edu-mx.zoom.us/j/7899086653?pwd=d2s0VXpIaCtHNy9haXhtVVZwK1dPQT09")
        .finish()
        .into_body()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/new_user").route(web::post().to(new_user)))
        .service(web::resource("/log_user").route(web::post().to(login_user)))
        .service(web::resource("/mission").route(web::post().to(handle_file)))
        .service(web::resource("/meet").route(web::get().to(meet)));
}
