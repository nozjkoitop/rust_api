use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    models::NewCar,
    services::car_service::CarService,
};


/// GET /cars
pub async fn list_cars(svc: web::Data<CarService>) -> impl Responder {
   
    let cars = match web::block(move || svc.list()).await {
        Ok(Ok(c))=> c,
        _ => return HttpResponse::InternalServerError().json("Something went wrong"),
    };
    HttpResponse::Ok().json(cars)
}

/// GET /cars/{id}
pub async fn get_car(
    svc: web::Data<CarService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();
    let car = match web::block(move || svc.get_by_id(id)).await {
        Ok(Ok(u)) => u,
        _ => return HttpResponse::NotFound().body(format!("There is no such car with {}",  id)).into(),
    };

    HttpResponse::Ok().json(car)
}

/// POST /cars
pub async fn create_car(
    svc: web::Data<CarService>,
    new_car: web::Json<NewCar>,
) -> impl Responder {
    let new_car = new_car.into_inner();
    let car = match web::block(move || svc.create(new_car)).await {
        Ok(Ok(c)) => c,
        _ => return HttpResponse::InternalServerError().body("Error creating car"),
    };

    HttpResponse::Created().json(car)
}
