use rocket::http::Status;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::users::model::{NewUserWithProject, User, UserExpanded};

use crate::app::modules::users::services::repository as user_repository;
use crate::app::modules::roles::services::repository as role_repository;
use crate::app::modules::user_project::services::repository as user_project_repository;

pub async fn helper_role_validation(
    db: &Db,
    user: &UserInClaims,
    new_user: &NewUserWithProject,
) -> Result<(), Status> {
    match user.role.name.as_str() {
        "admin" => Ok(()),
        "coord" => {
            if new_user.role_id <= user.role.id {
                return Err(Status::Unauthorized);
            }
            if new_user.depends_on != user.id {
                match user_repository::get_user_by_id(db, new_user.depends_on).await {
                    Ok(thera) => {
                        if thera.depends_on != user.id {
                            return Err(Status::Unauthorized);
                        }
                    }
                    Err(_) => return Err(Status::NotFound),
                }
            }

            Ok(())
        }
        "thera" => {
            if new_user.role_id <= user.role.id {
                return Err(Status::Unauthorized);
            }
            if new_user.depends_on != user.id {
                return Err(Status::Unauthorized);
            }

            Ok(())
        }
        _ => Err(Status::Unauthorized),
    }
}

pub async fn user_expanded_constructor(db: &Db, user: User) -> Result<UserExpanded, &'static str> {
    // get depends_on
    let depends_on = user_repository::get_user_by_id(&db, user.depends_on).await;
    if let Err(_) = depends_on {
        return Err("The user doesn't depend on a user that doesn't exist");
    }
    let depends_on = depends_on.unwrap();

    // get role
    let role = role_repository::get_role_by_id(&db, user.role_id).await;
    if let Err(_) = role {
        return Err("The user doesn't have a role that doesn't exist");
    }
    let role = role.unwrap();

    // get user_project
    let user_project = match user_project_repository::get_user_project_by_user_id(&db, user.id).await {
        Ok(user_project) => user_project,
        Err(e) => {
            println!("Error: {}", e);
            return Err("The user doesn't have a project");
        }
    };

    // build the user_expanded
    let user_expanded = UserExpanded {
        id: user.id,
        role,
        depends_on,
        user_token: user.user_token,
        project: user_project,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    Ok(user_expanded)
}

