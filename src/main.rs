use actix_files::Files;
use actix_web::{App, http, HttpResponse, HttpServer};
use actix_web::cookie::SameSite;
use actix_web::dev::Service;

//use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use futures::future::{Either, ok};

use actix_session::*;
use actix_web::http::header::HeaderValue;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    core::init_pool();
    core::sessions::init_sessions();

    let builder = core::ssl::load_ssl();

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);

                Box::pin(async move {
                    let mut res = fut.await?;
                    let headers = res.headers_mut();
                    headers.append(
                        actix_web::http::header::CACHE_CONTROL,
                        HeaderValue::from_str("no-cache").unwrap()
                    );
                    headers.append(
                        actix_web::http::header::CACHE_CONTROL,
                        HeaderValue::from_str("no-store").unwrap()
                    );
                    return Ok(res);
                })
            })
            .wrap_fn(|req, srv| {
            if req.connection_info().scheme() == "https"
            {
                Either::Left(srv.call(req))
            } else {
                let host = req.connection_info().host().to_owned();
                let uri = req.uri().to_owned();
                let url = format!("https://{}{}", host, uri);

                Either::Right(ok(req.into_response(
                    HttpResponse::TemporaryRedirect()
                        .insert_header((http::header::LOCATION, url))
                        .finish()
                )))

                }
            })
            .wrap_fn(|mut req, srv|{
                let _allowed_always = vec!["/new_user", "/log_user", "/register", "/login", "/new_user", "/log_user"  ];
                let only_with_logged_paths = vec!["/home" ];

                let session = req.get_session();

                if !only_with_logged_paths.contains(&req.path()) {
                    Either::Left(srv.call(req))
                } else {
                    let mut logged_in : bool = false;

                    if let Some(_) = req.get_session().get::<String>("session").unwrap() {
                        logged_in = core::sessions::is_user_logged_in(&session);
                    } else {
                        let to_insert = ("session".to_string(), serde_json::to_string(&"NONE".to_string()).unwrap());
                        Session::set_session(&mut req, vec![to_insert] );
                    }

                    let main_url = "/home";


                    if logged_in && req.path() == main_url {
                        let mut home = include_str!("../public/home/index.html").to_string();

                        core::handlers::get_mission_status(req.get_session());

                        {
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
                        }

                        Either::Right(ok(req.into_response(HttpResponse::Ok().body(home))))
                    } else if logged_in  {
                        Either::Left(srv.call(req))
                    } else {
                        let path = req.path().to_string();
                        let to_insert = ("goes-to".to_string(), serde_json::to_string(&path.to_string()).unwrap());
                        Session::set_session(&mut req, vec![to_insert]);
                        session.insert("goes-to",&path.to_string()).unwrap();

                        Either::Right(ok(req.into_response(
                            HttpResponse::Found()
                                .insert_header((http::header::LOCATION, "/login"))
                                .finish()
                        )))
                    }
                }
            })
            .wrap(CookieSession::signed(&[0; 128])
                .secure(true).http_only(false)
                .same_site(SameSite::Strict)
                .path("/")
                .name("session")
            )
            //.wrap(RedirectSchemeBuilder::new().build())
            .configure(core::routes)
            .service(Files::new("/", "/Users/alejandro/actix/public/").index_file("index.html"))
    })
        .bind("0.0.0.0:80")?
        .bind_rustls("0.0.0.0:443", builder.clone())?
        .run()
        .await
}