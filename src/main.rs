use actix_files::Files;
use actix_web::{App, HttpServer};
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

            .wrap_fn(|req: ServiceRequest, srv| {
                let fut = srv.call(req);

                Box::pin(async move {
                    let mut res: ServiceResponse = fut.await?;

                    let path: Option<String> = if res.request().path().contains("pdf") {
                        Some(res.request().path().to_string())
                    } else {
                        None
                    };

                    let headers = res.headers_mut();

                    headers.append(
                        actix_web::http::header::CACHE_CONTROL,
                        HeaderValue::from_str("no-cache").unwrap()
                    );

                    headers.append(
                        actix_web::http::header::CACHE_CONTROL,
                        HeaderValue::from_str("no-store").unwrap()
                    );

                    if let Some(_) = path {
                        headers.remove(actix_web::http::header::CONTENT_DISPOSITION);

                        headers.append(
                            actix_web::http::header::CONTENT_DISPOSITION,
                            HeaderValue::from_str("inline").unwrap()
                        );
                    }

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