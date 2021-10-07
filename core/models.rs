use serde::{Serialize, Deserialize};
use crate::diesel::*;
use crate::schema::*;

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="names"]
pub struct Info {
    pub name: String
}

#[derive(Queryable, Debug)]
pub struct Info2 {
    pub name_id: i64,
    pub name: String
}

#[derive(Queryable,  Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct Users {
    pub username: String,
    pub password: String,
    pub name: String
}

#[derive(Queryable,  Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct UsersLogin {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct UsersForm {
    pub username: String,
    pub password: String,
    pub password_repeat: String,
    pub name: String,
}