use crate::{models::{Image, NewImage}, repositories::image_repository::ImageRepository};
use std::error::Error;

#[derive(Clone)]
pub struct ImageService {
    repo: ImageRepository,
}

impl ImageService {
    pub fn new(repo: ImageRepository) -> Self {
        Self { repo }
    }

    pub fn list_for_car(&self, car_id: i64) -> Result<Vec<Image>, Box<dyn Error + Send + Sync>> {
        self.repo.list_for_car(car_id)
    }

    pub fn upload(&self, new: NewImage) -> Result<Image, Box<dyn Error + Send + Sync>> {
        // could add business logic (e.g. file‐storage) here
        self.repo.create(&new)
    }

    pub fn delete(&self, id: i64) -> Result<usize, Box<dyn Error + Send + Sync>> {
        self.repo.delete(id)
    }
}
