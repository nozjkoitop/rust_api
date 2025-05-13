use crate::{models::{NewUser, User}, repositories::user_repository::UserRepository};
use std::error::Error;
use std::io;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    pub fn register(&self, new: NewUser) -> Result<User, Box<dyn Error + Send + Sync>> {
        // e.g. check username uniqueness, hash password earlier…
        self.repo.create(&new)
    }
    
    pub fn get_by_username(&self, username: &str) -> Result<User, Box<dyn Error + Send + Sync>> {
        let opt = self.repo.find_by_username(username)?;
        opt.ok_or_else(|| {
            Box::new(io::Error::new(io::ErrorKind::NotFound, "Nu such user"))
                as Box<dyn Error + Send + Sync>
        })
    }
}
