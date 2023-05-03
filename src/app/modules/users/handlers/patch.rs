// extern
// use rocket::http::Status;

// app
// use crate::app::providers::interfaces::helpers::claims::UserInClaims;
// use crate::config::database::Db;

// module
// use crate::app::modules::users::services::repository as user_repository;

// pub async fn patch_fcm_user(
//     db: Db,
//     user: UserInClaims,
//     id: i32,
//     fcm_token: String,
// ) -> Result<Status, Status> {
//     if user.id != id {
//         return Err(Status::BadRequest);
//     }

//     match user_repository::update_fcm_token(&db, id, fcm_token).await {
//         Ok(_) => Ok(Status::Ok),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }
