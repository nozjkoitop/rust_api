use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{Car, NewCar};
use std::error::Error;

#[derive(Clone)]
pub struct CarRepository {
    pool: DbPool,
}

impl CarRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn list(&self) -> Result<Vec<Car>, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let items = crate::schema::cars::dsl::cars
            .load::<Car>(&mut conn)?;
        Ok(items)
    }

    pub fn get_by_id(&self, id: uuid::Uuid) -> Result<Car, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let car = crate::schema::cars::dsl::cars
            .find(id)
            .get_result::<Car>(&mut conn)?;
        Ok(car)
    }

    pub fn create(&self, new: &NewCar) -> Result<Car, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let car = diesel::insert_into(crate::schema::cars::dsl::cars)
            .values(new)
            .get_result::<Car>(&mut conn)?;
        Ok(car)
    }

    pub fn delete(&self, id: uuid::Uuid) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let affected = diesel::delete(crate::schema::cars::dsl::cars.find(id))
            .execute(&mut conn)?;
        Ok(affected)
    }
}
