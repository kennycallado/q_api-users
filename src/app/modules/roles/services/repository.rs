use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::roles;

use crate::app::modules::roles::model::Role;

pub async fn get_role_by_id(db: &Db, role_id: i32) -> Result<Role, diesel::result::Error> {
    let role = db
        .run(move |conn| roles::table.find(role_id).first::<Role>(conn))
        .await;

    role
}
