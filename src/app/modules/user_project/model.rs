use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::user_project;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::users::model::User;

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable, Associations))]
#[cfg_attr(feature = "db_diesel", diesel(belongs_to(User)))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = user_project))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserProject {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub active: bool,
    // pub keys: Vec<Option<String>>,
    pub keys: Vec<String>,
    pub record: Option<rocket::serde::json::Value>,
}

// #[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
// #[diesel(belongs_to(User))]
// #[diesel(table_name = user_project)]

#[cfg_attr(feature = "db_diesel", derive(Insertable, AsChangeset, Associations))]
#[cfg_attr(feature = "db_diesel", diesel(belongs_to(User)))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = user_project))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUserProject {
    pub user_id: i32,
    pub project_id: i32,
    pub active: Option<bool>,
    pub keys: Vec<Option<String>>,
    pub record: Option<rocket::serde::json::Value>,
}
