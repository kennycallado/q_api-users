// extern
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use rocket::State;

use crate::database::connection::Db;

use crate::app::providers::models::message::PubToken;
use crate::app::providers::models::project::PubProject;
use crate::app::providers::services::claims::UserInClaims;
use crate::app::providers::services::fetch::Fetch;

use crate::app::modules::user_project::model::NewUserProject;
use crate::app::modules::users::model::{NewUser, NewUserWithProject, User, UserExpanded};

use super::helper;

// repositories
use crate::app::modules::roles::services::repository as role_repository;
use crate::app::modules::user_project::services::repository as up_repository;
use crate::app::modules::users::services::repository as user_repository;

pub async fn create_user(
    fetch: &State<Fetch>,
    db: Db,
    user: UserInClaims,
    new_user: NewUserWithProject) -> Result<Json<UserExpanded>, Status> {
    match helper::helper_role_validation(&db, &user, &new_user).await {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    let project_id = new_user.project_id;
    let active = new_user.active;

    let user = match helper_add_db(&db, new_user.into()).await {
        Ok(user) => match helper_redirections(fetch, &db, project_id, active, user).await {
            Ok(user_exp) => user_exp,
            Err(e) => return Err(e),
        },
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(user))
}

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

async fn helper_redirections(
    fetch: &State<Fetch>,
    db: &Db,
    project_id: i32,
    active: Option<bool>,
    user: User) -> Result<UserExpanded, Status> {
    let project = match PubProject::init_user(fetch, project_id, user.id).await {
        Ok(project) => project,
        Err(e) => return Err(e),
    };

    let new_user_project = NewUserProject {
        user_id: user.id,
        project_id: project.id,
        active,
        keys: project.keys,
        record: None,
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

    // inicia user en message
    match PubToken::init_user(fetch, user.id).await {
        Ok(_) => {}
        Err(_) => return Err(Status::InternalServerError),
    }

    // devuelve UserExtended
    let user_exp = UserExpanded {
        id: user.id,
        depends_on,
        role,
        user_token: user.user_token,
        project,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(user_exp)
}
