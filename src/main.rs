use actix_files::Files;
use actix_web::{App, body, HttpServer};
use actix_web::cookie::SameSite;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};

use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;

use actix_session::*;
use actix_web::http::header::{HeaderValue};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    core::init_pool();
    core::sessions::init_sessions();

    let builder = core::ssl::load_ssl();

    HttpServer::new(|| {
        App::new()
            // Serve pdf files to Safari users
            .wrap_fn(|req: ServiceRequest, srv| {
                let fut = srv.call(req);

                Box::pin(async move {
                    let res: ServiceResponse = fut.await?;

                    if res.request().path() == "/p/covid" || res.request().path() == "/p/covid/"
                        && res.request().headers().contains_key(actix_web::http::header::USER_AGENT) {
                        let user_agent = res.request().headers()
                            .get(actix_web::http::header::USER_AGENT)
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();

                        if user_agent.to_lowercase().contains("safari")
                            && user_agent.to_lowercase().contains("version")
                            && !user_agent.to_lowercase().contains("chrome") {

                            let res_clone = res.request().clone();

                            let _body_data = match std::str::from_utf8(&body::to_bytes(res.into_body()).await?){
                                Ok(str) => {
                                    str.to_string()
                                }
                                Err(_) => {
                                    "Unknown".to_string()
                                }
                            };

                            let new_res = ServiceResponse::new(
                                res_clone,
                                actix_web::HttpResponse::Ok()
                                    .body(_body_data.replace(".png", ".pdf"))
                            );

                            return Ok(new_res);
                        }
                    }


                    return Ok(res);
                })
            })
            .wrap_fn(|req: ServiceRequest, srv| {
                let fut = srv.call(req);

                Box::pin(async move {
                    let mut res: ServiceResponse = fut.await?;

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
            .wrap(RedirectSchemeBuilder::new().build())
            .wrap(CookieSession::signed(&[0; 128])
                .secure(true).http_only(false)
                .same_site(SameSite::Strict)
                .path("/")
                .name("session")
            )
            .configure(core::routes)
            .service(Files::new("/", "/Users/alejandro/actix/public/").index_file("index.html"))

    })
        .bind("0.0.0.0:80")?
        .bind_rustls("0.0.0.0:443", builder.clone())?
        .run()
        .await
}