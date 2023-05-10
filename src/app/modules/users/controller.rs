use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::constants::ROBOT_TOKEN_EXPIRATION;
use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::config::database::Db;

use crate::app::modules::users::model::{NewUser, User, UserExpanded};
use crate::app::modules::users::services::repository as user_repository;

use super::handlers::create;
use super::handlers::index;
// use super::handlers::patch;
use super::handlers::show;
use super::handlers::update;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        options_claims,
        options_me,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        get_show_claims,
        get_show_claims_none,
        get_show_me,
        get_show_me_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/")]
pub async fn options_index() -> Status {
    Status::Ok
}

#[options("/<_id>")]
pub async fn options_show(_id: i32) -> Status {
    Status::Ok
}

#[options("/<_id>/userinclaims")]
pub async fn options_claims(_id: i32) -> Status {
    Status::Ok
}

#[options("/me")]
pub async fn options_me() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: Db, claims: AccessClaims) -> Result<Json<Vec<User>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        "coord" => index::get_index_coord(db, claims.0.user).await,
        "thera" => index::get_index_thera(db, claims.0.user).await,
        _ => {
            println!(
                "Error: get_index; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 1)]
pub async fn get_show(db: Db, claims: AccessClaims, id: i32) -> Result<Json<UserExpanded>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(db, claims.0.user, id).await,
        "coord" => show::get_show_coord(db, claims.0.user, id).await,
        "thera" => show::get_show_thera(db, claims.0.user, id).await,
        "user"  => show::get_show_user(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_show; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>", rank = 3)]
pub async fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/<id>/userinclaims", rank = 1)]
pub async fn get_show_claims(db: Db, claims: AccessClaims, id: i32) -> Result<Json<UserInClaims>, Status> {
    // Check if the token is a robot token
    if claims.0.iat + ROBOT_TOKEN_EXPIRATION != claims.0.exp {
        return Err(Status::Unauthorized);
    }

    match claims.0.user.role.name.as_str() {
        "robot" => show::get_show_robot(db, claims.0.user, id).await,
        _ => {
            println!(
                "Error: get_show_claims; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[get("/me", rank = 2)]
pub async fn get_show_me(db: Db, claims: AccessClaims) -> Result<Json<User>, Status> {
    let user = user_repository::get_user_by_id(&db, claims.0.user.id).await;

    match user {
        Ok(user) => {
            if user.active { Ok(Json(user)) } else { Err(Status::Unauthorized) }
        },
        Err(_) => {
            println!("Error: bla bla");
            Err(Status::InternalServerError)
        }
    }
}

#[get("/me", rank = 4)]
pub async fn get_show_me_none() -> Status {
    Status::Unauthorized
}

#[get("/<_id>/userinclaims", rank = 2)]
pub async fn get_show_claims_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<new_user>", rank = 1)]
pub async fn post_create(
    db: Db,
    claims: AccessClaims,
    new_user: Json<NewUser>,
) -> Result<Json<User>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(db, claims.0.user, new_user.into_inner()).await,
        "coord" => create::post_create_coord(db, claims.0.user, new_user.into_inner()).await,
        "thera" => create::post_create_thera(db, claims.0.user, new_user.into_inner()).await,
        _ => {
            println!(
                "Error: post_create; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[post("/", data = "<_new_user>", rank = 2)]
pub async fn post_create_none(_new_user: Json<NewUser>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<new_user>", rank = 1)]
pub async fn put_update(
    db: Db,
    claims: AccessClaims,
    id: i32,
    new_user: Json<NewUser>,
) -> Result<Json<User>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(db, claims.0.user, id, new_user.into_inner()).await,
        "coord" => update::put_update_coord(db, claims.0.user, id, new_user.into_inner()).await,
        "thera" => update::put_update_thera(db, claims.0.user, id, new_user.into_inner()).await,
        _ => {
            println!(
                "Error: put_update; Role not handled {}",
                claims.0.user.role.name
            );
            Err(Status::BadRequest)
        }
    }
}

#[put("/<_id>", data = "<_new_user>", rank = 2)]
pub async fn put_update_none(_id: i32, _new_user: Json<NewUser>) -> Status {
    Status::Unauthorized
}

// #[patch("/<id>/fcm", data = "<fcm_token>", rank = 1)]
// pub async fn patch_fcm_token(
//     db: Db,
//     claims: AccessClaims,
//     id: i32,
//     fcm_token: Json<String>,
// ) -> Result<Status, Status> {
//     match claims.0.user.role.name.as_str() {
//         "admin" => patch::patch_fcm_user(db, claims.0.user, id, fcm_token.into_inner()).await,
//         "coord" => patch::patch_fcm_user(db, claims.0.user, id, fcm_token.into_inner()).await,
//         "thera" => patch::patch_fcm_user(db, claims.0.user, id, fcm_token.into_inner()).await,
//         "user" => patch::patch_fcm_user(db, claims.0.user, id, fcm_token.into_inner()).await,
//         _ => {
//             println!(
//                 "Error: patch_update_fcm_user; Role not handled {}",
//                 claims.0.user.role.name
//             );
//             Err(Status::BadRequest)
//         }
//     }
// }

// #[patch("/<_id>/fcm", data = "<_fcm_token>", rank = 2)]
// pub async fn patch_fcm_token_none(_id: i32, _fcm_token: Json<String>) -> Status {
//     Status::Unauthorized
// }
