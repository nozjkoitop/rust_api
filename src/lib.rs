pub mod db;
pub mod models;
pub mod schema;

pub mod auth {
    pub mod jwt;
    pub mod middleware;
}

pub mod config {
    pub mod jwt_config;
}

pub mod repositories {
    pub mod car_repository;
    pub mod user_repository;
    pub mod image_repository;
}

pub mod services {
    pub mod car_service;
    pub mod user_service;
    pub mod image_service;
}

pub mod handlers {
    pub mod auth_handlers;
    pub mod car_handlers;
    pub mod image_handlers;
}
