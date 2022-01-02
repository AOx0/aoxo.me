use std::io::BufReader;
use rustls_pemfile::{certs, pkcs8_private_keys};
use rustls::{Certificate, PrivateKey, ServerConfig};


pub fn load_ssl() -> ServerConfig {
    // convertí el .p12 a .crt y .key con la herramienta en namecheap
    // const CERT: &'static [u8] = include_bytes!("../e74df7b05aa2050a55bb18d000d4102.crt");
    const KEY: &'static [u8] = include_bytes!("../e74df7b05aa2050a55bb18d000d4102.key");
    const CHAIN: &'static [u8] = include_bytes!("../aoxo_me.ca-bundle");

    // let mut cert = BufReader::new(CERT);
    let mut chain = BufReader::new(CHAIN);
    let mut key = BufReader::new(KEY);

    let cert_chain = certs(&mut chain).unwrap();
    let mut keys = pkcs8_private_keys(&mut key).unwrap();

    let k = PrivateKey(keys.remove(0));

    let mut c: Vec<Certificate> = Vec::new();
    for certificate in cert_chain {
        c.push(Certificate(certificate))
    }

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(c, k)
        .unwrap()
}