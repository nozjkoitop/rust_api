use actix_web::{web, HttpResponse, Responder};

use crate::{models::NewCar, services::car_service::CarService};

/// GET /cars
pub async fn list_cars(svc: web::Data<CarService>) -> impl Responder {
    let cars = match web::block(move || svc.list()).await {
        Ok(Ok(c)) => c,
        Ok(Err(e)) => {
            return HttpResponse::InternalServerError().body(format!("Error listing cars: {}", e));
        }
        Err(block_err) => {
            log::error!("blocking error: {:?}", block_err);
            return HttpResponse::InternalServerError()
                .body(format!("Error listing cars: {}", block_err));
        }
    };
    HttpResponse::Ok().json(cars)
}

/// GET /cars/{id}
pub async fn get_car(svc: web::Data<CarService>, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let car = match web::block(move || svc.get_by_id(id)).await {
        Ok(Ok(u)) => u,
        Ok(Err(e)) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error getting car with id {}: {}", id, e));
        }
        Err(block_err) => {
            log::error!("blocking error: {:?}", block_err);
            return HttpResponse::InternalServerError()
                .body(format!("Error getting car with id {}: {}", id, block_err));
        }
    };

    HttpResponse::Ok().json(car)
}

/// POST /cars
pub async fn create_car(svc: web::Data<CarService>, new_car: web::Json<NewCar>) -> impl Responder {
    let new_car = new_car.into_inner();
    let car = match web::block(move || svc.create(new_car)).await {
        Ok(Ok(c)) => c,
        Ok(Err(e)) => {
            return HttpResponse::InternalServerError().body(format!("Error creating car: {}", e));
        }
        Err(block_err) => {
            log::error!("blocking error: {:?}", block_err);
            return HttpResponse::InternalServerError()
                .body(format!("Error creating car: {}", block_err));
        }
    };

    HttpResponse::Created().json(car)
}
