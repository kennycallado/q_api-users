#[cfg(feature = "db_diesel")]
use diesel::prelude::*;

#[cfg(feature = "db_diesel")]
use crate::database::schema::user_project;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;
#[cfg(feature = "db_sqlx")]
use sqlx::QueryBuilder;

use crate::app::providers::models::record::PubNewRecord;
use crate::app::modules::user_project::model::{NewUserProject, UserProject};
use crate::database::connection::Db;

pub async fn get_user_project_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<UserProject, sqlx::Error> {
    let user_project: UserProject = 
        sqlx::query_as!(UserProject, "SELECT * FROM user_project WHERE user_id = $1", user_id).fetch_one(&db.0).await?;

    Ok(user_project)
}

pub async fn get_user_projects_active_user_project_by_project_id(
    db: &Db,
    project_id: i32,
) -> Result<Vec<UserProject>, sqlx::Error> {
    let user_project = sqlx::query_as!(UserProject, "SELECT * FROM user_project WHERE project_id = $1 AND active = true", project_id).fetch_all(&db.0).await?;

    Ok(user_project)
}

pub async fn create_user_project(
    db: &Db,
    new_user_project: NewUserProject,
) -> Result<UserProject, sqlx::Error> {
    let user_project = sqlx::query_as!(
        UserProject,
        "INSERT INTO user_project (user_id, project_id, active, keys, record) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        new_user_project.user_id,
        new_user_project.project_id,
        new_user_project.active,
        &new_user_project.keys,
        new_user_project.record
    ).fetch_one(&db.0).await?;

    Ok(user_project)
}

pub async fn update_user_record(db: &Db, new_record: PubNewRecord) -> Result<usize, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE user_project SET record = $1 WHERE user_id = $2",
        new_record.record, new_record.user_id
    ).execute(&db.0).await?.rows_affected();

    Ok(result as usize)
}

pub async fn toggle_active(db: &Db, user_id: i32) -> Result<usize, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE user_project SET active = NOT active WHERE user_id = $1",
        user_id
    ).execute(&db.0).await?.rows_affected();

    Ok(result as usize)
}
