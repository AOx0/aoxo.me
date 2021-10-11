//! Cookie-based session tools
//!
//! Does not create cookies by itself but stores methods to generate new id session strings.
//!
//! `USERS_NAMES`, a Mutex<HashMap<String, String>> that stores by session_id, user all users that are logged in
//! `USERS` a  Mutex<Vec<String>> that stores all session ids that are logged in. `main` function reads them and checks if the user has a cookie equal to one of them.

use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::random;

use crate::diesel::prelude::*;
use crate::{pool, schema};

lazy_static! {
    pub static ref USERS_NAMES: Mutex<HashMap<String, String>> = {
         Mutex::new(HashMap::new())
    };
}

lazy_static! {
    pub static ref USERS: Mutex<Vec<String>> = {
         Mutex::new(vec![])
    };
}

pub fn get_user_name(session: String) -> Option<String> {
    let dict =  USERS_NAMES.lock().unwrap();

    return if dict.contains_key(&*session) {
        Some(dict.get(&session).unwrap().clone())
    } else {
        None
    }

}

/// Adds a user's name to USERS_NAMES
pub fn associate(user: String, session: String) {
    let mut dict =  USERS_NAMES.lock().unwrap();
    dict.insert(session, user);
}

/// Generates a random 64-char string. Used to identify users (only when logged in)
/// No session_key is generated for non logged in users
pub fn generate_new_session_cookie() -> String {
    (0..64).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

pub fn add_session(session: String) {
    USERS.lock().unwrap().push(session);
}

pub fn is_user_registered(session: String) -> bool {
    USERS.lock().unwrap().contains(&session)
}

pub fn init_sessions() {
    lazy_static::initialize(&USERS);
    lazy_static::initialize(&USERS_NAMES);
}

pub fn get_user_id_in_db(username: String) -> i64 {
    use schema::users::dsl as n;

    if let Ok(id) = n::users.select(n::id)
        .filter(n::username.eq(username.to_ascii_uppercase()))
        .first::<i64>(&pool::connect()) {
        id
    } else {
        0
    }
}