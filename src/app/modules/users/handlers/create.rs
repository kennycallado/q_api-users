// extern
use rocket::http::Status;
use rocket::State;
use rocket::serde::{json::Json, uuid::Uuid};
use serde::Serialize;

use crate::app::modules::user_project::model::NewUserProject;
use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;
use crate::app::providers::interfaces::project::PubProject;
use crate::config::database::Db;

// app
use crate::app::providers::interfaces::helpers::claims::UserInClaims;
use crate::app::providers::interfaces::helpers::fetch::Fetch;

// module
use crate::app::modules::users::model::{NewUser, NewUserWithProject, User, UserExpanded};

use crate::app::modules::users::services::repository as user_repository;
use crate::app::modules::user_project::services::repository as up_repository;
use crate::app::modules::roles::services::repository as role_repository;

async fn helper_add_db(db: &Db, new_user: NewUser) -> Result<User, ()> {
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

pub async fn post_create_admin(fetch: &State<Fetch>, db: Db, _admin: UserInClaims, new_user: NewUserWithProject)
-> Result<Json<UserExpanded>, Status> {
    let project_id = new_user.project_id;

    let user = match helper_add_db(&db, new_user.into()).await {
        Ok(user) => {
            // Envia nuevo usuario a projects
            // recibe info project
            let project = match helper_add_project(fetch, project_id, user.id).await {
                Ok(project) => project,
                Err(e) => return Err(e),
            };

            let new_user_project = NewUserProject {
                user_id: user.id,
                project_id: project.id,
                keys: project.keys,
                values: rocket::serde::json::Value::String("{}".to_string()),
            };

            let project = match up_repository::create_user_project(&db, new_user_project).await {
                Ok(user_project) => user_project,
                Err(_) => return Err(Status::InternalServerError),
            };

            let role = match role_repository::get_role_by_id(&db, user.role_id).await {
                Ok(role) => role,
                Err(_) => return Err(Status::InternalServerError),
            };

            let depends_on = match user_repository::get_user_by_id(&db, user.depends_on).await {
                Ok(user) => user,
                Err(_) => return Err(Status::InternalServerError),
            };

            // agrega fcm
            match helper_add_fcm(fetch, user.id).await {
                Err(_) => {},
                Ok(_)  => {},
            }
            
            // devuelve UserExtended
            let user_exp = UserExpanded {
                id: user.id,
                depends_on,
                role,
                user_token: user.user_token,
                active: user.active,
                project,
                created_at: user.created_at,
                updated_at: user.updated_at
            };
            
            user_exp
        }
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(user))
}

async fn helper_add_fcm(fetch: &State<Fetch>, user_id: i32) -> Result<(), Status> {
    #[derive(Serialize)]
    struct NewToken {
        user_id: i32,
        token: Option<String>
    }

    let new_token = NewToken {user_id, token: None};

    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let fcm_url = ConfigGetter::get_entity_url("fcm").unwrap_or("http://localhost:8005/api/v1/fcm".to_string())
        + "/token";

    let client = fetch.client.lock().await;
    let res = client
        .post(fcm_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&new_token)
        .send()
        .await;

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            // Ok(res.json::<PubProject>().await.unwrap())
            Ok(())
        }
        Err(_) => Err(Status::InternalServerError)
    }
}

async fn helper_add_project(fetch: &State<Fetch>, project_id: i32, user_id: i32) -> Result<PubProject, Status> {
    let robot_token = match Fetch::robot_token().await {
        Ok(token) => token,
        Err(_) => return Err(Status::InternalServerError),
    };

    let project_url = ConfigGetter::get_entity_url("project").unwrap_or("http://localhost:8051/api/v1/project".to_string())
        + "/"
        + project_id.to_string().as_str()
        + "/user/"
        + user_id.to_string().as_str()
        + "/new";

    let client = fetch.client.lock().await;
    let res = client
        .get(project_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .send()
        .await;

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<PubProject>().await.unwrap())
        }
        Err(_) => Err(Status::InternalServerError)
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

    match helper_add_db(&db, new_user.into()).await {
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

    match helper_add_db(&db, new_user.into()).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
