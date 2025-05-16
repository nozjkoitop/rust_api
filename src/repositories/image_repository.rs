use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{Image, NewImage};
use std::error::Error;
use crate::schema::images::dsl::{images, car_id};

#[derive(Clone)]
pub struct ImageRepository {
    pool: DbPool,
}

impl ImageRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }


    pub fn list_for_car(&self, car: i64) -> Result<Vec<Image>, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let items = images
            .into_boxed::<diesel::pg::Pg>()
            .filter(car_id.eq(car))
            .load::<Image>(&mut conn)?;  
        Ok(items)
    }

    pub fn create(&self, new: &NewImage) -> Result<Image, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let img = diesel::insert_into(images)
            .values(new)
            .get_result::<Image>(&mut conn)?;
        Ok(img)
    }

    pub fn delete(&self, id: i64) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let affected = diesel::delete(images.find(id))
            .execute(&mut conn)?;
        Ok(affected)
    }

    pub fn get(&self, id: i64) -> Result<Image, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        Ok(images.find(id).first::<Image>(&mut conn)?)
    }
}
