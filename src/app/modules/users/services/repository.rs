// externs
use diesel::prelude::*;

// app
use crate::config::database::Db;
use crate::database::schema::users;

// module
use crate::app::modules::users::model::{NewUser, User};

pub async fn get_all(db: &Db) -> Result<Vec<User>, diesel::result::Error> {
    let users = db.run(move |conn| users::table.load::<User>(conn)).await;

    users
}

pub async fn get_user_by_id(db: &Db, id: i32) -> Result<User, diesel::result::Error> {
    let user = db
        .run(move |conn| users::table.find(id).first::<User>(conn))
        .await;

    user
}

pub async fn get_users_by_depend(
    db: &Db,
    depends_on: i32,
) -> Result<Vec<User>, diesel::result::Error> {
    let users = db
        .run(move |conn| {
            users::table
                .filter(users::depends_on.eq(depends_on))
                .load::<User>(conn)
        })
        .await;

    users
}

pub async fn add_user(db: &Db, new_user: NewUser) -> Result<User, diesel::result::Error> {
    let user = db
        .run(move |conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .get_result(conn)
        })
        .await;

    user
}

pub async fn update_user(
    db: &Db,
    id: i32,
    new_user: NewUser,
) -> Result<User, diesel::result::Error> {
    let user = db
        .run(move |conn| {
            diesel::update(users::table.find(id))
                .set(new_user)
                .get_result(conn)
        })
        .await;

    user
}

pub async fn update_user_token(
    db: &Db,
    id: i32,
    user_token: String,
) -> Result<String, diesel::result::Error> {
    let user = db
        .run(move |conn| {
            diesel::update(users::table.find(id))
                .set(users::user_token.eq(user_token))
                .get_result::<User>(conn)
        })
        .await;

    if let Err(e) = user {
        return Err(e);
    }
    let user: User = user.unwrap();

    Ok(user.user_token.unwrap()) // Almost sure I can unwrap here
}

// pub async fn update_fcm_token(
//     db: &Db,
//     id: i32,
//     fcm_token: String,
// ) -> Result<String, diesel::result::Error> {
//     let user = db
//         .run(move |conn| {
//             diesel::update(users::table.find(id))
//                 .set(users::fcm_token.eq(fcm_token))
//                 .get_result::<User>(conn)
//         })
//         .await;

//     if let Err(e) = user {
//         return Err(e);
//     }
//     let user: User = user.unwrap();

//     Ok(user.fcm_token.unwrap()) // Almost sure I can unwrap here
// }
