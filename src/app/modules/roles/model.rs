use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::answers;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Role {
    pub id: i32,
    pub name: String,
}
