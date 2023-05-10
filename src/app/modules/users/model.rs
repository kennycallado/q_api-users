use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::schema::users;

use crate::app::modules::roles::model::Role;
use crate::app::modules::user_project::model::UserProject;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(User, foreign_key = depends_on))]
#[diesel(treat_none_as_null = true)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub depends_on: i32,
    pub role_id: i32,
    pub user_token: Option<String>,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User, foreign_key = depends_on))]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
#[serde(crate = "rocket::serde")]
pub struct UserExpanded {
    pub id: i32,
    pub depends_on: User,
    pub role: Role,
    pub user_token: Option<String>,
    pub active: bool,
    pub project: Option<UserProject>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Insertable, Associations, AsChangeset)]
#[diesel(belongs_to(User, foreign_key = depends_on))]
#[diesel(table_name = users)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub depends_on: i32,
    pub role_id: i32,
    pub active: bool,
}

impl From<User> for NewUser {
    fn from(user: User) -> Self {
        NewUser {
            depends_on: user.depends_on,
            role_id: user.role_id,
            active: user.active,
        }
    }
}
