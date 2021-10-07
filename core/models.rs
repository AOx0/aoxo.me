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

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct Users {
    pub username: String,
    pub password: String
}