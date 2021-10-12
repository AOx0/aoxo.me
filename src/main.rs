use actix_files::Files;
use actix_web::{App, http, HttpResponse, HttpServer, web};
use actix_web::cookie::SameSite;
use actix_web::dev::Service;

use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use futures::future::{Either, ok};

use actix_session::*;
use core::handlers::get_mission_status;
use core::sessions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    core::init_pool();
    sessions::init_sessions();

    let builder = core::ssl::load_ssl();

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|mut req, srv|{
                let urls = vec!["/", "/register", "/home","/login" ];

                let session = req.get_session();

                if !urls.contains(&req.path()) {
                    Either::Left(srv.call(req))
                } else {
                    let mut logged_in : bool = false;

                    if let Some(_) = req.get_session().get::<String>("session").unwrap() {
                        logged_in = sessions::is_user_logged_in(&session);
                    } else {
                        let to_insert = ("session".to_string(), serde_json::to_string(&"NONE".to_string()).unwrap());
                        Session::set_session(vec![to_insert], &mut req);
                    }

                    if logged_in && req.path() == "/home" {
                        let mut home = include_str!("../public/home/index.html").to_string();

                        get_mission_status(req.get_session());


                        if req.get_session().get::<bool>("mission1").unwrap().unwrap() {
                            home = home.replace("Mission 1", "Mission 1 ✔");
                        }

                        if req.get_session().get::<bool>("mission2").unwrap().unwrap() {
                            home = home.replace("Mission 2", "Mission 2 ✔");
                        }

                        if req.get_session().get::<bool>("mission3").unwrap().unwrap() {
                            home = home.replace("Mission 3", "Mission 3 ✔");
                        }

                        if req.get_session().get::<bool>("mission4").unwrap().unwrap() {
                            home = home.replace("Mission 4", "Mission 4 ✔");
                        }

                        if req.get_session().get::<bool>("mission5").unwrap().unwrap() {
                            home = home.replace("Mission 5", "Mission 5 ✔");
                        }

                        Either::Right(ok(req.into_response(
                            HttpResponse::Ok()
                                .body(home)

                        )))
                    } else if logged_in  {
                        if req.path() != "/register"  && req.path() != "/login" {
                            Either::Left(srv.call(req))
                        } else {
                            Either::Right(ok(req.into_response(
                                HttpResponse::Found()
                                        .header(http::header::LOCATION, "/home")
                                        .finish()
                                        .into_body(),
                            )))
                        }
                    } else {
                        // Don't forward to /login if we are already on /login

                        if req.path() == "/login" || req.path() == "/register" {
                            Either::Left(srv.call(req))
                        } else {
                            Either::Right(ok(req.into_response(
                                HttpResponse::Found()
                                        .header(http::header::LOCATION, "/login")
                                        .finish()
                                        .into_body(),
                            )))
                        }
                    }
                }
            })
            .wrap(CookieSession::signed(&[0; 128])
                .secure(true).http_only(false)
                .same_site(SameSite::Strict)
                .path("/")
                .name("session")
            )
            .wrap(RedirectSchemeBuilder::new().build())
            .configure(core::routes)
            .service(
                web::scope("/")
                    .wrap(core::cache_middleware::MyCacheInterceptor)
                    .service(Files::new("", "./public/.").show_files_listing().index_file("index.html"))
            )
    })
        .bind("0.0.0.0:80")?
        .bind_rustls("0.0.0.0:443", builder.clone())?
        .run()
        .await
}