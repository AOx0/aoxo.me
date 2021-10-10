use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::Mutex;
use rand::random;

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

pub fn associate(user: String, session: String) {
    let mut dict =  USERS_NAMES.lock().unwrap();
    dict.insert(session, user);
}

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