use rocket::http::Status;
use rocket::serde::json::Json;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::users::model::{User, UserExpanded};
use crate::app::modules::users::handlers::helper;

use crate::app::modules::roles::services::repository as role_repository;
use crate::app::modules::user_project::services::repository as user_project_repository;
use crate::app::modules::users::services::repository as user_repository;

pub async fn get_show_admin(db: Db, _admin: UserInClaims, id: i32) -> Result<Json<UserExpanded>, Status> {
    let user = match user_repository::get_user_by_id(&db, id).await {
        Ok(user) => user,
        Err(_) => return Err(Status::NotFound),
    };

    match helper::user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(e) => {
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        },
    }
}

pub async fn get_show_coord(db: Db, coord: UserInClaims, id: i32) -> Result<Json<UserExpanded>, Status> {
    // get user
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

    match helper::user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(e) => {
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        },
    }
}

pub async fn get_show_thera(db: Db, thera: UserInClaims, id: i32) -> Result<Json<UserExpanded>, Status> {
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

    match helper::user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(e) => {
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        },
    }
}

pub async fn get_show_user(db: Db, user_claims: UserInClaims, id: i32) -> Result<Json<UserExpanded>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    if user_claims.id != user.id {
        return Err(Status::Unauthorized);
    }

    match helper::user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(e) => {
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        },
    }
}

// Should check or update the user_token?
pub async fn get_show_robot(db: Db, _robot: UserInClaims, id: i32) -> Result<Json<UserInClaims>, Status> {
    let user = match user_repository::get_user_by_id(&db, id).await {
        Ok(user) => match helper::user_expanded_constructor(&db, user).await {
            Ok(user_expanded) => Ok(Json(user_expanded.into())),
            Err(e) => {
                println!("Error: {}", e);
                Err(Status::InternalServerError)
            },
        },
        Err(_) => return Err(Status::NotFound),
    };

    user
}
