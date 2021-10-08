use std::io::BufReader;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

pub fn load_ssl() -> ServerConfig {
    // convertí el .p12 a .crt y .key con la herramienta en namecheap
    // const CERT: &'static [u8] = include_bytes!("../e74df7b05aa2050a55bb18d000d4102.crt");
    const KEY: &'static [u8] = include_bytes!("../e74df7b05aa2050a55bb18d000d4102.key");
    const CHAIN: &'static [u8] = include_bytes!("../aoxo_me.ca-bundle");

    // let mut cert = BufReader::new(CERT);
    let mut chain = BufReader::new(CHAIN);
    let mut key = BufReader::new(KEY);

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_chain = certs(&mut chain).unwrap();

    let mut keys = pkcs8_private_keys(&mut key).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}