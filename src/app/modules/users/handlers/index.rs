use rocket::http::Status;
use rocket::serde::json::Json;

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::user_project::model::UserProject;
use crate::app::modules::users::model::User;
use crate::app::providers::models::record::{PubRecord, PubNewRecord};
use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::user_project::services::repository as user_project_repository;
use crate::app::modules::users::services::repository as user_repository;

pub async fn get_index_admin(db: &Db, _user: UserInClaims) -> Result<Json<Vec<User>>, Status> {
    let users = user_repository::get_all(db).await;

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::NotFound),
    }
}

pub async fn get_index_records(db: &Db, _user: UserInClaims, project_id: i32) -> Result<Json<Vec<PubNewRecord>>, Status> {
    let user_projects = user_project_repository::get_user_projects_active_user_project_by_project_id(db, project_id).await;

    match user_projects {
        Ok(user_projects) => {
            let mut records = Vec::new();
            for up in user_projects {
                let record = PubNewRecord {
                    user_id: up.user_id,
                    record: up.record,
                };

                records.push(record)
            }

            Ok(Json(records))
        },
        Err(_) => Err(Status::NotFound),
    }

}

pub async fn get_index_coord(db: &Db, user: UserInClaims) -> Result<Json<Vec<User>>, Status> {
    let mut response = Vec::new();
    let therapists = user_repository::get_users_by_depend(db, user.id).await;

    if let Err(_) = therapists {
        return Err(Status::NotFound);
    }
    let therapists = therapists.unwrap();

    for therapist in therapists {
        let users = user_repository::get_users_by_depend(db, therapist.id).await;
        if let Err(_) = users {
            continue;
        }
        let users = users.unwrap();

        // Add the therapist to the list
        response.push(therapist);

        // Add the users of the therapist to the list
        response.extend(users);
    }

    Ok(Json(response))
}
pub async fn get_index_thera(db: &Db, user: UserInClaims) -> Result<Json<Vec<User>>, Status> {
    let users = user_repository::get_users_by_depend(db, user.id).await;

    if let Err(_) = users {
        return Err(Status::NotFound);
    }
    let users = users.unwrap();

    Ok(Json(users))
}
