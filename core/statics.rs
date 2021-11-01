use lazy_static::lazy_static;

pub fn init_pool() { lazy_static::initialize(&CORREOS) }

lazy_static! {
    pub static ref CORREOS: Vec<String> = {
        Vec::from(["pabloalejandroortizm@gmail.com".to_string()])
    };
}