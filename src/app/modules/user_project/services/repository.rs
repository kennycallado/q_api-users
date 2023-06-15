use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::user_project;

use crate::app::providers::models::record::PubNewRecord;

use crate::app::modules::user_project::model::{NewUserProject, UserProject};

pub async fn get_user_project_by_user_id(
    db: &Db,
    user_id: i32,
) -> Result<UserProject, diesel::result::Error> {
    let user_project = db
        .run(move |conn| {
            user_project::table
                .filter(user_project::user_id.eq(user_id))
                .first::<UserProject>(conn)
        })
        .await;

    user_project
}

pub async fn create_user_project(
    db: &Db,
    new_user_project: NewUserProject,
) -> Result<UserProject, diesel::result::Error> {
    let user_project = db
        .run(move |conn| {
            diesel::insert_into(user_project::table)
                .values(new_user_project)
                .get_result::<UserProject>(conn)
        })
        .await;

    user_project
}

pub async fn update_user_record(db: &Db, new_record: PubNewRecord) -> Result<usize, diesel::result::Error> {
    let result = db
        .run(move |conn| {
            diesel::update(user_project::table)
                .filter(user_project::user_id.eq(new_record.user_id))
                .set(user_project::record.eq(new_record.record))
                .execute(conn)
        })
        .await;

    result
}
