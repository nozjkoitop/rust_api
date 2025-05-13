use actix_web::{web, HttpResponse, Responder};
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
) -> impl Responder {
    // 1) Hash the plaintext password
    let hashed = match hash(&info.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // 2) Build our Insertable NewUser
    let new_user = NewUser {
        username: info.username.clone(),
        email: info.email.clone(),
        password_hash: hashed,
        role: UserRole::User, // default role
    };

    // 3) Call the service on a thread pool
    let created = match web::block(move || user_svc.register(new_user)).await {
        Ok(Ok(u)) => u,
        _ => return HttpResponse::Unauthorized().body("Bad credentials"),
    };
    
    HttpResponse::Created().json(created)
}

/// POST /auth/login

pub async fn login(
    user_svc: web::Data<UserService>,
    jwt_mgr: web::Data<JwtManager>,
    info: web::Json<LoginRequest>,
) -> impl Responder {
    let username = info.username.clone();

    let user = match web::block(move || user_svc.get_by_username(&username)).await {
        Ok(Ok(u)) => u,
        _ => return HttpResponse::Unauthorized().body("Bad credentials"),
    };

    // 2) Verify password
    if !verify(&info.password, &user.password_hash).unwrap_or(false) {
        return HttpResponse::Unauthorized().body("Bad credentials");
    }

    // 3) Issue JWT
    match jwt_mgr.create_token(&user.id.to_string()) {
        Ok(token) => HttpResponse::Ok().json(AuthResponse { token }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
