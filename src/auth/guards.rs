use actix_web::{HttpMessage, HttpRequest, Error as ActixError, error};
use crate::auth::jwt::Claims;
use crate::models::user::UserRole;

pub fn require_admin(req: &HttpRequest) -> Result<(), ActixError> {
    match req.extensions().get::<Claims>().map(|c| &c.role) {
        Some(UserRole::Admin) => Ok(()),
        _ => Err(error::ErrorForbidden("admin privileges required")),
    }
}