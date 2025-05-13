use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{User, NewUser};
use crate::schema::users::dsl::{users, username};
use std::error::Error;

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn find_by_username(&self, username_str: &str) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        
        let user = users
            .filter(username.eq(username_str))
            .first::<User>(&mut conn)
            .optional()?; 
        Ok(user)
    }

    pub fn create(&self, new: &NewUser) -> Result<User, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let user = diesel::insert_into(crate::schema::users::dsl::users)
            .values(new)
            .get_result::<User>(&mut conn)?;
        Ok(user)
    }

    // add more: get_by_id, update_role, delete, etc.
}
