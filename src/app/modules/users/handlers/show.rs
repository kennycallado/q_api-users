// extern
use rocket::http::Status;
use rocket::serde::json::Json;

// app
use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

// module
use crate::app::modules::users::model::User;
use crate::app::modules::users::services::repository as user_repository;

pub async fn get_show_admin(
    db: Db,
    _admin: UserInClaims,
    id: i32,
) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    Ok(Json(user))
}

pub async fn get_show_coord(
    db: Db,
    coord: UserInClaims,
    id: i32,
) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user.role_id {
        2 => {
            // The user is a coord so the coord should be the same
            if user.id != coord.id {
                return Err(Status::Unauthorized);
            }
        }
        3 => {
            // The user is a thera so validate that the thera depends on the coord
            if user.depends_on != coord.id {
                println!("The user doesn't depend on the coord");
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // Validate that the user depends on a therapist of the coord
            let therapist = user_repository::get_user_by_id(&db, user.depends_on).await;

            if therapist.is_err() {
                println!("The user depends on a therapist that doesn't exist");
                return Err(Status::NotFound);
            }
            let therapist = therapist.unwrap();

            if therapist.depends_on != coord.id {
                println!("The user does't depend on a therapist of the coord");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    Ok(Json(user))
}

pub async fn get_show_thera(
    db: Db,
    thera: UserInClaims,
    id: i32,
) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user.role_id {
        3 => {
            // The user is a thera so the thera should be the same
            if user.id != thera.id {
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // Validate that the user depends on the therapist
            if user.depends_on != thera.id {
                println!("The user does't depend on a this therapist");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    Ok(Json(user))
}

pub async fn get_show_user(
    db: Db,
    user_claims: UserInClaims,
    id: i32,
) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    if user_claims.id != user.id {
        return Err(Status::Unauthorized);
    }

    Ok(Json(user))
}

// Should check or update the user_token?
pub async fn get_show_robot(
    db: Db,
    _robot: UserInClaims,
    id: i32,
) -> Result<Json<UserInClaims>, Status> {
    let user = user_repository::get_user_expanded_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    Ok(Json(user.into()))
}
