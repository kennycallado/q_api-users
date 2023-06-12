use rocket::http::Status;

use crate::database::connection::Db;

use crate::app::providers::services::claims::UserInClaims;

use crate::app::modules::users::model::NewUserWithProject;
use crate::app::modules::users::services::repository as user_repository;

pub async fn helper_role_validation(db: &Db, user: &UserInClaims, new_user: &NewUserWithProject) -> Result<(), Status> {
    match user.role.name.as_str() {
        "admin" => Ok(()),
        "coord" => {
            if new_user.role_id <= user.role.id { return Err(Status::Unauthorized) }
            if new_user.depends_on != user.id {
                match user_repository::get_user_by_id(db, new_user.depends_on).await {
                    Ok(thera) => {
                        if thera.depends_on != user.id { return Err(Status::Unauthorized) }
                    }
                    Err(_) => return Err(Status::NotFound),
                }
            }

            Ok(())
        },
        "thera" => {
            if new_user.role_id <= user.role.id { return Err(Status::Unauthorized) }
            if new_user.depends_on != user.id { return Err(Status::Unauthorized) }

            Ok(())
        },
        _ => Err(Status::Unauthorized),
    }
}
