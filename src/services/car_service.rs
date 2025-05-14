use crate::{models::{Car, NewCar}, repositories::car_repository::CarRepository};
use std::error::Error;

#[derive(Clone)]
pub struct CarService {
    repo: CarRepository,
}

impl CarService {
    pub fn new(repo: CarRepository) -> Self {
        Self { repo }
    }

    pub fn list(&self) -> Result<Vec<Car>, Box<dyn Error + Send + Sync>> {
        self.repo.list()
    }

    pub fn get_by_id(&self, id: i64) -> Result<Car, Box<dyn Error + Send + Sync>> {
        self.repo.get_by_id(id)
    }

    pub fn create(&self, new: NewCar) -> Result<Car, Box<dyn Error + Send + Sync>> {
        self.repo.create(&new)
    }

    pub fn delete(&self, id: i64) -> Result<usize, Box<dyn Error + Send + Sync>> {
        self.repo.delete(id)
    }
}