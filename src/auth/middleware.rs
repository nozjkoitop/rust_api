use actix_web::{dev::ServiceRequest, Error as ActixError};
use actix_web_httpauth::{
    extractors::bearer::BearerAuth,
    middleware::HttpAuthentication,
};
use crate::auth::jwt::JwtManager;
use std::future::{ready, Ready};
use actix_web::web::Data;

fn validator(
    req: ServiceRequest,
    creds: BearerAuth,
) -> Ready<Result<ServiceRequest, (ActixError, ServiceRequest)>> {
    // look up Data<JwtManager> instead of JwtManager
    if let Some(jwt_data) = req.app_data::<Data<JwtManager>>() {
        let jwt_mgr: &JwtManager = jwt_data.get_ref();
        if jwt_mgr.verify_token(creds.token()).is_ok() {
            ready(Ok(req))
        } else {
            ready(Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)))
        }
    } else {
        ready(Err((actix_web::error::ErrorInternalServerError(
            "JwtManager not configured",
        ), req)))
    }
}

pub fn auth_middleware(
) -> HttpAuthentication<
    BearerAuth,
    fn(ServiceRequest, BearerAuth) -> Ready<Result<ServiceRequest, (ActixError, ServiceRequest)>>,
> {
    HttpAuthentication::bearer(validator)
}