use diesel::prelude::*;
use crate::db::DbPool;
use crate::models::{Car, NewCar};
use std::error::Error;
use crate::schema::cars::dsl::cars;

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
        let items = cars
            .load::<Car>(&mut conn)?;
        Ok(items)
    }

    pub fn get_by_id(&self, id: i64) -> Result<Car, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let car = cars
            .find(id)
            .get_result::<Car>(&mut conn)?;
        Ok(car)
    }

    pub fn create(&self, new: &NewCar) -> Result<Car, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let car = ::diesel::insert_into(cars)
            .values(new)
            .get_result::<Car>(&mut conn)?;
        Ok(car)
    }

    pub fn delete(&self, id: i64) -> Result<usize, Box<dyn Error + Send + Sync>> {
        let mut conn = self.pool.get()?;
        let affected = diesel::delete(cars.find(id))
            .execute(&mut conn)?;
        Ok(affected)
    }
}
