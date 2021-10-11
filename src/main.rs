use actix::ActorTryFutureExt;
use actix_files::Files;
use actix_web::{App, http, HttpResponse, HttpServer, web};
use actix_web::cookie::SameSite;
use actix_web::dev::Service;

use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use futures::future::{Either, ok};

use actix_session::*;
use diesel::BoolExpressionMethods;
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

                if !urls.contains(&req.path()) {
                    Either::Left(srv.call(req))
                } else {
                    let mut logged_in : bool = false;

                    if let Some(key) = req.get_session().get::<String>("session").unwrap() {
                        logged_in = sessions::is_user_registered(key);
                    } else {
                        let to_insert = ("session".to_string(), serde_json::to_string(&"NONE".to_string()).unwrap());
                        Session::set_session(vec![to_insert], &mut req);
                    }

                    if logged_in && req.path() == "/home" {
                        let home = include_str!("../public/home/index.html");

                        Either::Right(ok(req.into_response(
                            HttpResponse::Ok()
                                .body(home.replace("Mission", "Hola"))

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