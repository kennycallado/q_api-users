use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "db_diesel")]
use crate::database::schema::users;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;
#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::roles::model::Role;
use crate::app::modules::user_project::model::UserProject;

#[cfg_attr(feature = "db_diesel", derive(Queryable, Identifiable, Associations))]
#[cfg_attr(feature = "db_diesel", diesel(belongs_to(Role)))]
#[cfg_attr(feature = "db_diesel", diesel(belongs_to(User, foreign_key = depends_on)))]
#[cfg_attr(feature = "db_diesel", diesel(treat_none_as_null = true))]
#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub depends_on: i32,
    pub role_id: i32,
    pub user_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserExpanded {
    pub id: i32,
    pub depends_on: User,
    pub role: Role,
    pub user_token: Option<String>,
    pub project: UserProject,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg_attr(feature = "db_diesel", derive(Insertable, AsChangeset, Associations))]
#[cfg_attr(feature = "db_diesel", diesel(table_name = users))]
#[cfg_attr(feature = "db_diesel", diesel(belongs_to(User, foreign_key = depends_on)))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUser {
    pub depends_on: i32,
    pub role_id: i32,
}

impl From<User> for NewUser {
    fn from(user: User) -> Self {
        NewUser {
            depends_on: user.depends_on,
            role_id: user.role_id,
        }
    }
}

impl From<NewUserWithProject> for NewUser {
    fn from(user: NewUserWithProject) -> Self {
        NewUser {
            depends_on: user.depends_on,
            role_id: user.role_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewUserWithProject {
    pub depends_on: i32,
    pub role_id: i32,
    pub active: Option<bool>,
    pub project_id: i32,
}
