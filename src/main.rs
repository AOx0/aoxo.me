use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    core::init_pool();

    HttpServer::new(|| {
        App::new()
            .configure(core::routes)
            .service(Files::new("/", "./public/").index_file("index.html"))
    })
        .bind("0.0.0.0:80")?
        .bind("localhost:80")?
        .bind("[::0]:80")?
        .run()
        .await
}