#[cfg(feature = "db_diesel")]
use diesel::prelude::*;

#[cfg(feature = "db_diesel")]
use crate::database::schema::users;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;
#[cfg(feature = "db_sqlx")]
use sqlx::QueryBuilder;

use crate::app::modules::users::model::{NewUser, User};
use crate::database::connection::Db;

pub async fn get_all(db: &Db) -> Result<Vec<User>, sqlx::Error> {
    let users: Vec<User> = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&db.0)
        .await?;

    Ok(users)
}

pub async fn get_user_by_id(db: &Db, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&db.0)
        .await?;

    Ok(user)
}

pub async fn get_users_by_depend(db: &Db, depends_on: i32) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, "SELECT * FROM users WHERE depends_on = $1", depends_on)
        .fetch_all(&db.0)
        .await?;

    Ok(users)
}

pub async fn add_user(db: &Db, new_user: NewUser) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "INSERT INTO users (depends_on, role_id) VALUES ($1, $2) RETURNING *", new_user.depends_on, new_user.role_id)
        .fetch_one(&db.0)
        .await?;
    Ok(user)
}

pub async fn update_user(db: &Db, id: i32, new_user: NewUser) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "UPDATE users SET depends_on = $1, role_id = $2 WHERE id = $3 RETURNING *", new_user.depends_on, new_user.role_id, id)
        .fetch_one(&db.0)
        .await?;

    Ok(user)
}

pub async fn update_user_token(
    db: &Db,
    id: i32,
    user_token: String,
) -> Result<String, sqlx::Error> {
    let user = sqlx::query_as!(User, "UPDATE users SET user_token = $1 WHERE id = $2 RETURNING *", user_token, id)
        .fetch_one(&db.0)
        .await?;

    Ok(user.user_token.unwrap())
}
