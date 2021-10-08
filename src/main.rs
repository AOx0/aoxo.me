extern crate actix_web;
extern crate rustls;
extern crate actix;


use actix_files::Files;
use actix_web::{App, HttpServer};
use std::io::BufReader;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};


fn load_ssl() -> ServerConfig {
    // const CERT: &'static [u8] = include_bytes!("../e74df7b05aa2050a55bb18d000d4102.crt");
    const KEY: &'static [u8] = include_bytes!("../e74df7b05aa2050a55bb18d000d4102.key");
    const CHAIN: &'static [u8] = include_bytes!("../aoxo_me.ca-bundle");

    // let mut cert = BufReader::new(CERT);
    let mut chain = BufReader::new(CHAIN );
    let mut key = BufReader::new(KEY);


    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_chain = certs(&mut chain).unwrap();

    let mut keys = pkcs8_private_keys(&mut key).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    core::init_pool();

    let builder = load_ssl();


    HttpServer::new(|| {
        App::new()
            .configure(core::routes)
            .service(Files::new("/", "./public/").index_file("index.html"))
    })
        .bind("0.0.0.0:80")?
        .bind_rustls("0.0.0.0:443", builder)?
        .run()
        .await
}