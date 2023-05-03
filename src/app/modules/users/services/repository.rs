// externs
use diesel::prelude::*;

// app
use crate::config::database::Db;
use crate::database::schema::roles;
use crate::database::schema::users;

// module
use crate::app::modules::roles::model::Role;
use crate::app::modules::users::model::{NewUser, User, UserExpanded};

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

pub async fn get_user_expanded_by_id(
    db: &Db,
    id: i32,
) -> Result<UserExpanded, diesel::result::Error> {
    let user_expanded = db
        .run(move |conn| {
            // Option to do it with inner join
            // and then ask for the depends_on user
            // let user_role = roles::table
            //     .inner_join(users::table)
            //     .filter(users::id.eq(id))
            //     .first::<(Role, User)>(conn);

            // let user_role = user_role.unwrap();

            let user = users::table.filter(users::id.eq(id)).first::<User>(conn);

            let user = user.unwrap();

            let role = roles::table
                .filter(roles::id.eq(user.role_id))
                .first::<Role>(conn);

            let role = role.unwrap();

            let depends_on = users::table
                .filter(users::id.eq(user.depends_on))
                .first::<User>(conn);
            let depends_on = depends_on.unwrap();

            UserExpanded {
                id: user.id,
                depends_on,
                role,
                user_token: user.user_token,
                // fcm_token: user.fcm_token,
                active: user.active,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }
        })
        .await;

    Ok(user_expanded)
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
