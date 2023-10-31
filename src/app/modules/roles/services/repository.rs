#[cfg(feature = "db_diesel")]
use diesel::prelude::*;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

#[cfg(feature = "db_diesel")]
use crate::database::schema::roles;

use crate::app::modules::roles::model::Role;
use crate::database::connection::Db;

pub async fn get_role_by_id(db: &Db, role_id: i32) -> Result<Role, sqlx::Error> {
    let role = sqlx::query_as!(Role, "SELECT * FROM roles WHERE id = $1", role_id)
        .fetch_one(&db.0).await?;

    Ok(role)
}
