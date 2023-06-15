use serde::{Deserialize, Serialize};

use crate::database::schema::user_project;

use crate::app::modules::users::model::User;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_project)]
#[serde(crate = "rocket::serde")]
pub struct UserProject {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub keys: Vec<Option<String>>,
    pub record: rocket::serde::json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = user_project)]
#[serde(crate = "rocket::serde")]
pub struct NewUserProject {
    pub user_id: i32,
    pub project_id: i32,
    pub keys: Vec<Option<String>>,
    pub record: rocket::serde::json::Value,
}
