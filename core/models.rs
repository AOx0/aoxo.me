use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub name: String
}