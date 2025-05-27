use actix_web::{dev::ServiceRequest, Error as ActixError, HttpMessage};
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
) -> Ready<Result<ServiceRequest,(ActixError,ServiceRequest)>> {
    match req.app_data::<Data<JwtManager>>() {
        Some(jwt) => match jwt.verify_token(creds.token()) {
            Ok(data) => {
                req.extensions_mut().insert(data.claims); 
                ready(Ok(req))
            }
            Err(_) => ready(Err((actix_web::error::ErrorUnauthorized("invalid token"), req))),
        },
        None => ready(Err((actix_web::error::ErrorInternalServerError("JwtManager missing"), req))),
    }
}


pub fn auth_middleware(
) -> HttpAuthentication<
    BearerAuth,
    fn(ServiceRequest, BearerAuth) -> Ready<Result<ServiceRequest, (ActixError, ServiceRequest)>>,
> {
    HttpAuthentication::bearer(validator)
}