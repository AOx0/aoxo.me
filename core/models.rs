use serde::{Serialize, Deserialize};
use crate::diesel::*;
use crate::schema::*;
use crate::diesel::Identifiable;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i64
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

#[derive(Serialize, Deserialize)]
pub struct File {
    pub file: String,
    pub file_id: String
}

#[derive(Queryable)]
pub struct Missions {
    pub mission1: bool,
    pub mission2: bool,
    pub mission3: bool,
    pub mission4: bool,
    pub mission5: bool,
}

#[derive(Queryable,  Insertable)]
#[table_name="missions"]
pub struct NewMission {
    pub user_id: i64,
}