// extern
use rocket::http::Status;
use rocket::serde::json::Json;

// app
use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

// module
use crate::app::modules::users::model::{User, UserExpanded};

use crate::app::modules::roles::services::repository as role_repository;
use crate::app::modules::user_project::services::repository as user_project_repository;
use crate::app::modules::users::services::repository as user_repository;

async fn user_expanded_constructor(db: &Db, user: User) -> Result<UserExpanded, Status> {
    // get depends_on
    let depends_on = user_repository::get_user_by_id(&db, user.depends_on).await;
    if let Err(_) = depends_on {
        return Err(Status::NotFound);
    }
    let depends_on = depends_on.unwrap();

    // get role
    let role = role_repository::get_role_by_id(&db, user.role_id).await;
    if let Err(_) = role {
        return Err(Status::NotFound);
    }
    let role = role.unwrap();

    // get user_project
    let user_project = match user_project_repository::get_user_project_by_user_id(&db, user.id).await {
        Ok(user_project) => Some(user_project),
        Err(_) => None,
    };

    // build the user_expanded
    let user_expanded = UserExpanded {
        id: user.id,
        role,
        depends_on,
        user_token: user.user_token,
        active: user.active,
        project: user_project,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(user_expanded)
}

pub async fn get_show_admin(db: Db, _admin: UserInClaims, id: i32) -> Result<Json<UserExpanded>, Status> {
    // get user
    let user = user_repository::get_user_by_id(&db, id).await;
    if let Err(_) = user {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(status) => Err(status),
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

    match user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(status) => Err(status),
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

    match user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(status) => Err(status),
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

    match user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded)),
        Err(status) => Err(status),
    }
}

// Should check or update the user_token?
pub async fn get_show_robot(db: Db, _robot: UserInClaims, id: i32) -> Result<Json<UserInClaims>, Status> {
    let user = user_repository::get_user_by_id(&db, id).await;

    if user.is_err() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    match user_expanded_constructor(&db, user).await {
        Ok(user_expanded) => Ok(Json(user_expanded.into())),
        Err(status) => Err(status),
    }
}
