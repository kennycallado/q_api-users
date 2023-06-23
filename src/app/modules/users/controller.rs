use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::app::providers::models::record::{PubNewRecord, PubRecord};
use crate::database::connection::Db;

use crate::app::providers::constants::ROBOT_TOKEN_EXPIRATION;
use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::users::handlers::{create, index, show, update};
use crate::app::modules::users::model::{NewUser, NewUserWithProject, User, UserExpanded};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_index,
        options_show,
        options_claims,
        options_me,
        get_index,
        get_index_none,
        get_index_records,
        get_index_records_none,
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
        post_update_record,
        post_update_record_none,
        get_update_toggle_active,
        get_update_toggle_active_none,
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
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[get("/", rank = 2)]
pub async fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/project/<project_id>/record", rank = 1)]
pub async fn get_index_records(db: Db, claims: AccessClaims, project_id: i32) -> Result<Json<Vec<PubNewRecord>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_records(db, claims.0.user, project_id).await,
        "robot" => index::get_index_records(db, claims.0.user, project_id).await,
        _ => {
            println!("Error: get_index_records; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[get("/project/<_project_id>/record", rank = 2)]
pub async fn get_index_records_none(_project_id: i32) -> Status {
    Status::Unauthorized
}


#[get("/<id>", rank = 1)]
pub async fn get_show(db: Db, claims: AccessClaims, id: i32) -> Result<Json<UserExpanded>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(db, claims.0.user, id).await,
        "coord" => show::get_show_coord(db, claims.0.user, id).await,
        "thera" => show::get_show_thera(db, claims.0.user, id).await,
        "user" => show::get_show_user(db, claims.0.user, id).await,
        _ => {
            println!("Error: get_show; Role not handled {}", claims.0.user.role.name);
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

#[get("/<_id>/userinclaims", rank = 2)]
pub async fn get_show_claims_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[get("/me", rank = 2)]
pub async fn get_show_me(db: Db, claims: AccessClaims) -> Result<Json<UserExpanded>, Status> {
    let id = claims.0.user.id;

    match show::get_show_admin(db, claims.0.user, id).await {
        Ok(user) => {
            match user.user_token {
                Some(_) => Ok(user),
                None => Err(Status::Unauthorized),
            }
        }
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

#[post("/", data = "<new_user>", rank = 1)]
pub async fn post_create(
    fetch: &State<Fetch>,
    db: Db,
    claims: AccessClaims,
    new_user: Json<NewUserWithProject>,
) -> Result<Json<UserExpanded>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::create_user(fetch, db, claims.0.user, new_user.into_inner()).await,
        "coord" => create::create_user(fetch, db, claims.0.user, new_user.into_inner()).await,
        "thera" => create::create_user(fetch, db, claims.0.user, new_user.into_inner()).await,
        _ => {
            println!("Error: post_create; Role not handled {}", claims.0.user.role.name);
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
            println!("Error: put_update; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[put("/<_id>", data = "<_new_user>", rank = 2)]
pub async fn put_update_none(_id: i32, _new_user: Json<NewUser>) -> Status {
    Status::Unauthorized
}

#[patch("/record", data = "<new_record>", rank = 1)]
pub async fn post_update_record(db: Db, claims: AccessClaims, new_record: Json<PubNewRecord>) -> Result<Status, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::post_update_record_admin(&db, claims.0.user, new_record.into_inner()).await,
        "robot" => update::post_update_record_admin(&db, claims.0.user, new_record.into_inner()).await,
        _ => {
            println!("Error: post_update_record; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[patch("/record", data = "<_new_record>", rank = 2)]
pub async fn post_update_record_none(_new_record: Json<PubNewRecord>) -> Status {
    Status::Unauthorized
}

#[get("/<id>/project/toggle", rank = 1)]
pub async fn get_update_toggle_active(db: Db, claims: AccessClaims, id: i32) -> Result<Status, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::get_udpate_user_toggle_active(&db, claims.0.user, id).await,
        "robot" => update::get_udpate_user_toggle_active(&db, claims.0.user, id).await,
        _ => {
            println!("Error: get_update_toggle_active; Role not handled {}", claims.0.user.role.name);
            Err(Status::BadRequest)
        }
    }
}

#[get("/<_id>/project/toggle", rank = 2)]
pub async fn get_update_toggle_active_none(_id: i32) -> Status {
    Status::Unauthorized
}
