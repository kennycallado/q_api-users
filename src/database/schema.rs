// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    user_project (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
        keys -> Array<Nullable<Text>>,
        values -> Jsonb,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        depends_on -> Int4,
        role_id -> Int4,
        user_token -> Nullable<Varchar>,
        active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(roles, user_project, users,);
