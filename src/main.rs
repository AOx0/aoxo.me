use actix_files::Files;
use actix_web::{App, HttpServer};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    core::init_pool();

    let builder = core::ssl::load_ssl();

    HttpServer::new(|| {
        App::new()
            .wrap(RedirectSchemeBuilder::new().build())
            .configure(core::routes)
            .service(Files::new("/", "./public/").index_file("index.html"))
    })
        .bind("0.0.0.0:80")?
        .bind_rustls("0.0.0.0:443", builder.clone())?
        .run()
        .await
}