// extern
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid};

use crate::config::database::Db;

// app
use crate::app::providers::interfaces::helpers::claims::UserInClaims;

// module
use crate::app::modules::users::model::{NewUser, NewUserWithProject, User};
use crate::app::modules::users::services::repository as user_repository;

async fn helper(db: &Db, new_user: NewUser) -> Result<User, ()> {
    match user_repository::add_user(db, new_user).await {
        Ok(user) => {
            let new_user_token = Uuid::new_v4().to_string();

            match user_repository::update_user_token(&db, user.id, new_user_token).await {
                Ok(user_token) => {
                    let mut user = user;
                    user.user_token = Some(user_token);

                    Ok(user)
                }
                Err(_) => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}

pub async fn post_create_admin(
    db: Db,
    _admin: UserInClaims,
    new_user: NewUserWithProject,
) -> Result<Json<User>, Status> {
    // waiting to call the api
    let _project_id = new_user.project_id;

    match helper(&db, new_user.into()).await {
        Ok(user) => {
            // call the projects_api to get the project details
            // construct the userproject and save it
            Ok(Json(user))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn post_create_coord(
    db: Db,
    coord: UserInClaims,
    new_user: NewUserWithProject,
) -> Result<Json<User>, Status> {
    match new_user.role_id {
        3 => {
            // Creating a therapist
            // Validate that the new_user depends on the coord
            if new_user.depends_on != coord.id {
                println!("The new_user does't depend on the coord");
                return Err(Status::Unauthorized);
            }
        }
        4 => {
            // Creating a patient
            // Validate that the new_user depends on a therapist of the coord
            let therapist = user_repository::get_user_by_id(&db, new_user.depends_on).await;

            if therapist.is_err() {
                println!("The new_user depends on a therapist that doesn't exist");
                return Err(Status::NotFound);
            }
            let therapist = therapist.unwrap();

            if therapist.depends_on != coord.id {
                println!("The new_user does't depend on a therapist of the coord");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    match helper(&db, new_user.into()).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn post_create_thera(
    db: Db,
    thera: UserInClaims,
    new_user: NewUserWithProject,
) -> Result<Json<User>, Status> {
    match new_user.role_id {
        4 => {
            // Creating a patient
            // Validate that the new_user depends on the thera
            if new_user.depends_on != thera.id {
                println!("The new_user does't depend on the thera");
                return Err(Status::Unauthorized);
            }
        }
        _ => return Err(Status::Unauthorized),
    }

    match helper(&db, new_user.into()).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
