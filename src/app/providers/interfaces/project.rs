use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubProject {
    pub id: i32,
    pub name: String,
    pub keys: Vec<Option<String>>,
}
