#[macro_use] extern crate diesel;

pub use handlers::routes;
pub use pool::init_pool;
pub mod models;
pub mod pool;
pub mod schema;
pub mod ssl;
pub mod sessions;
//pub mod cache_middleware;
pub mod handlers;
pub mod statics;


