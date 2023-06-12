use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::users::model::User;
use crate::app::modules::users::services::repository as user_repository;

pub async fn get_index_admin(db: Db, _user: UserInClaims) -> Result<Json<Vec<User>>, Status> {
    let users = user_repository::get_all(&db).await;

    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::NotFound),
    }
}

pub async fn get_index_coord(db: Db, user: UserInClaims) -> Result<Json<Vec<User>>, Status> {
    let mut response = Vec::new();
    let therapists = user_repository::get_users_by_depend(&db, user.id).await;

    if let Err(_) = therapists {
        return Err(Status::NotFound);
    }
    let therapists = therapists.unwrap();

    for therapist in therapists {
        let users = user_repository::get_users_by_depend(&db, therapist.id).await;
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
pub async fn get_index_thera(db: Db, user: UserInClaims) -> Result<Json<Vec<User>>, Status> {
    let users = user_repository::get_users_by_depend(&db, user.id).await;

    if let Err(_) = users {
        return Err(Status::NotFound);
    }
    let users = users.unwrap();

    Ok(Json(users))
}
