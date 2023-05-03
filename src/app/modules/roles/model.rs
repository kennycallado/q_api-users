use serde::{Deserialize, Serialize};

use crate::database::schema::roles;

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Role {
    pub id: i32,
    pub name: String,
}
