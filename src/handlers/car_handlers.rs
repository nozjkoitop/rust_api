use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult, error};

use crate::{models::NewCar, services::car_service::CarService, auth::guards::require_admin};

/// GET /cars
pub async fn list_cars(svc: web::Data<CarService>) -> ActixResult<HttpResponse> {
    let cars = web::block(move || svc.list())
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cars))
}

/// GET /cars/{id}
pub async fn get_car(svc: web::Data<CarService>, path: web::Path<i64>) -> ActixResult<HttpResponse> {
    let id = path.into_inner();
    let car = web::block(move || svc.get_by_id(id))
        .await
        .map_err(error::ErrorInternalServerError)?    
        .map_err(|e| error::ErrorInternalServerError(
            format!("Error getting car {}: {}", id, e)
        ))?;

    Ok(HttpResponse::Ok().json(car))
}

/// POST /cars
pub async fn create_car(req: HttpRequest, svc: web::Data<CarService>, new_car: web::Json<NewCar>) -> ActixResult<HttpResponse> {
    require_admin(&req)?;
    let car = web::block(move || svc.create(new_car.into_inner()))
        .await?
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(car))
}

/// DELETE /cars/{id}
pub async fn delete_car(
    req: HttpRequest,
    svc: web::Data<CarService>,
    path: web::Path<i64>,
) -> ActixResult<HttpResponse> {
    require_admin(&req)?;
    let id = path.into_inner();

    let affected = web::block(move || svc.delete(id))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    if affected == 0 {
        Ok(HttpResponse::NotFound().finish())
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}
