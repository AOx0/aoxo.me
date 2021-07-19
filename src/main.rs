use actix_web::{App, HttpServer};
use actix_files::Files;
use core::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes)
            .service(Files::new("/", "./public/").index_file("index.html"))
    })
        .bind("0.0.0.0:8080")?
        .bind("[::0]:8080")?
        .run()
        .await
}