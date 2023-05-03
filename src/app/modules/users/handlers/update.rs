// extern
use rocket::http::Status;
use rocket::serde::json::Json;

// app
use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

// module
use crate::app::modules::users::model::{NewUser, User};
use crate::app::modules::users::services::repository as user_repository;

pub async fn put_update_admin(
    db: Db,
    admin: UserInClaims,
    id: i32,
    new_user: NewUser,
) -> Result<Json<User>, Status> {
    match new_user.role_id {
        1 => {
            // updating an admin
            // Validate that the admin is the same
            if admin.id != id {
                return Err(Status::Unauthorized);
            }
        }
        _ => {}
    }

    let user = user_repository::update_user(&db, id, new_user).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn put_update_coord(
    db: Db,
    coord: UserInClaims,
    id: i32,
    new_user: NewUser,
) -> Result<Json<User>, Status> {
    match new_user.role_id {
        // Coord can't update another coord
        // 2 => {
        //     // updating a coord
        //     // Validate that the coord is the same
        //     if coord.id != id {
        //         return Err(Status::Unauthorized);
        //     }
        // }
        3 => {
            // updating a therapist
            // Validate that the therapist depends on the coord
            if new_user.depends_on != coord.id {
                println!("The therapist does't depend on the coord");
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // updating a patient
            // Validate that the patient depends on a therapist of the coord
            match user_repository::get_user_by_id(&db, new_user.depends_on).await {
                Ok(therapist) => {
                    if therapist.depends_on != coord.id {
                        println!("The patient does't depend on a therapist of the coord");
                        return Err(Status::Unauthorized);
                    }
                }
                Err(_) => {
                    println!("The patient depends on a therapist that doesn't exist");
                    return Err(Status::NotFound);
                }
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    let user = user_repository::update_user(&db, id, new_user).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

// The only reason is because thera can make a patient active
pub async fn put_update_thera(
    db: Db,
    thera: UserInClaims,
    id: i32,
    new_user: NewUser,
) -> Result<Json<User>, Status> {
    match new_user.role_id {
        4 => {
            // Updating a user
            // Validate that the user depends on the therapist
            if new_user.depends_on != thera.id {
                println!("The patient does't depend on the therapist");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }
    let user = user_repository::update_user(&db, id, new_user).await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
