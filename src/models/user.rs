use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Clone, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::UserRole"]
#[DbValueStyle = "PascalCase"]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    #[diesel(sql_type = UserRole)]
    pub role: UserRole,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    #[diesel(column_name = role, sql_type = UserRole)]
    pub role: UserRole,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
