use actix_web::{web, HttpResponse, Result as ActixResult, error};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Serialize;

use crate::{
    auth::jwt::JwtManager,
    models::user::{LoginRequest, NewUser, RegisterRequest, UserRole},
    services::user_service::UserService,
};

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

/// POST /auth/register
pub async fn register(
    user_svc: web::Data<UserService>,
    info: web::Json<RegisterRequest>,
) -> ActixResult<HttpResponse> {
    let hashed = hash(&info.password, DEFAULT_COST)
        .map_err(error::ErrorInternalServerError)?;
    let new_user = NewUser {
        username: info.username.clone(),
        phone: info.phone.clone(),
        email: info.email.clone(),
        password_hash: hashed,
        role: UserRole::User,
    };

    let created = web::block(move || user_svc.register(new_user))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(created))
}

/// POST /auth/login
pub async fn login(
    user_svc: web::Data<UserService>,
    jwt_mgr: web::Data<JwtManager>,
    info: web::Json<LoginRequest>,
) -> ActixResult<HttpResponse> {
    let username = info.username.clone();

    let user = web::block(move || user_svc.get_by_username(&username))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(|_| error::ErrorUnauthorized("Bad credentials"))?;

    if !verify(&info.password, &user.password_hash).unwrap_or(false) {
        return Err(error::ErrorUnauthorized("Bad credentials"));
    }

    let token = jwt_mgr
        .create_token(&user.id, &user.role.clone())
        .map_err(|_| error::ErrorInternalServerError("Token creation failed"))?;

    Ok(HttpResponse::Ok().json(AuthResponse { token }))
}
