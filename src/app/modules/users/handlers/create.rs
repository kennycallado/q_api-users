// extern
use rocket::http::Status;
use rocket::serde::{json::Json, uuid::Uuid};

// app
use crate::app::providers::interfaces::helpers::claims::{
    // Claims,
    UserInClaims
};
// use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;
use crate::config::database::Db;

// module
use crate::app::modules::users::model::{NewUser, User};
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

// async fn robot_token_generator() -> Result<String, ()> {
//     let mut claims: Claims = Claims::from(UserInClaims::default());

//     let access_token = claims.enconde_for_robot();
//     if let Err(_) = access_token {
//         return Err(());
//     };

//     match access_token {
//         Ok(access_token) => Ok(access_token),
//         Err(_) => Err(()),
//     }
// }

// async fn helper_enable_fcm(user: &User) -> Result<Status, Status> {
//     // TODO: Refactor this
//     use serde::{Deserialize, Serialize};
//     #[derive(Serialize, Deserialize)]
//     struct NewFcmToken {
//         // HasMap ??
//         user_id: i32,
//         fcm_token: Option<String>,
//     }
//     // Create new_fcm_token
//     let new_fcm_token = NewFcmToken {
//         user_id: user.id,
//         fcm_token: None,
//     };

//     // Create robot_token
//     let robot_token = match robot_token_generator().await {
//         Ok(robot_token) => robot_token,
//         Err(_) => return Err(Status::InternalServerError),
//     };

//     // Get the api_url
//     let fcm_api_url = ConfigGetter::get_fcm_url()
//         .unwrap_or("http://localhost:8005/api/v1/fcm".to_string())
//         + "/token/";

//     // Send the request to the fcm api
//     let client = reqwest::Client::new();
//     let res = client
//         .post(&fcm_api_url)
//         .header("Accept", "application/json")
//         .header("Authorization", robot_token)
//         .header("Content-Type", "application/json")
//         .json(&new_fcm_token)
//         .send()
//         .await;

//     // Validate the response
//     match res {
//         Ok(res) => {
//             // if res.status() != 201 {
//             if !res.status().to_string().starts_with('2') {
//                 println!("Error creating the fcm token");
//                 return Err(Status::from_code(res.status().as_u16()).unwrap());
//             }

//             Ok(Status::Ok)
//         }
//         Err(_) => {
//             println!("Error creating the fcm token");

//             Err(Status::InternalServerError)
//         }
//     }
// }

pub async fn post_create_admin(
    db: Db,
    _admin: UserInClaims,
    new_user: NewUser,
) -> Result<Json<User>, Status> {
    match helper(&db, new_user).await {
        Ok(user) => {
            // Create the fcm token
            // match helper_enable_fcm(&user).await {
            //     Ok(_) => (),
            //     Err(_) => return Err(Status::InternalServerError),
            // }
            Ok(Json(user))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn post_create_coord(
    db: Db,
    coord: UserInClaims,
    new_user: NewUser,
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

    match helper(&db, new_user).await {
        Ok(user) => {
            // Create the fcm token
            // match helper_enable_fcm(&user).await {
            //     Ok(_) => (),
            //     Err(_) => return Err(Status::InternalServerError),
            // }
            Ok(Json(user))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn post_create_thera(
    db: Db,
    thera: UserInClaims,
    new_user: NewUser,
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

    match helper(&db, new_user).await {
        Ok(user) => {
            // Create the fcm token
            // match helper_enable_fcm(&user).await {
            //     Ok(_) => (),
            //     Err(_) => return Err(Status::InternalServerError),
            // }
            Ok(Json(user))
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
