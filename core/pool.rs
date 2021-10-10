//! Pool of connections to postgres database. .env must be set with `DATABASE_URL`
//! 
//! Crate `r2d2` generates the pool of connections. We can access one with `.get()`
//! 
//! A lazy_static ref is used to have global full-execution living data where connections are stored
//! ```rust
//! lazy_static! {
//!     static ref POOL: Pool = {
//!         dotenv::dotenv().ok();
//!         let url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL");
//!         let manager = ConnectionManager::new(url);
//!         Pool::new(manager).expect("Failed to create pool")
//!     };
//! }
//! ```


use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        dotenv::dotenv().ok();
        let url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL");
        let manager = ConnectionManager::new(url);
        Pool::new(manager).expect("Failed to create pool")
    };
}

pub fn connect() -> Connection {POOL.get().unwrap()}
pub fn init_pool() { lazy_static::initialize(&POOL) }