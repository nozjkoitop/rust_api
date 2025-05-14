// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    cars (id) {
        id -> Int8,
        make -> Varchar,
        model -> Varchar,
        year -> Int4,
        price -> Numeric,
        created_at -> Timestamp,
    }
}

diesel::table! {
    images (id) {
        id -> Int8,
        car_id -> Int8,
        url -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int8,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        role -> UserRole,
        created_at -> Timestamp,
    }
}

diesel::joinable!(images -> cars (car_id));

diesel::allow_tables_to_appear_in_same_query!(
    cars,
    images,
    users,
);
