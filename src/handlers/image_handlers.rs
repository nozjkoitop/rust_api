use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    models::NewImage,
    services::image_service::ImageService,
};

/// GET /cars/{car_id}/images
pub async fn list_images(
    svc: web::Data<ImageService>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let car_id = path.into_inner();

    let imgs = match web::block(move || svc.list_for_car(car_id)).await {
        Ok(Ok(c)) => c,
        _ => return HttpResponse::InternalServerError().body("Error listing images"),
    };
    HttpResponse::Ok().json(imgs)
}

/// POST /cars/{car_id}/images
pub async fn upload_image(
    svc: web::Data<ImageService>,
    path: web::Path<Uuid>,
    new_img: web::Json<NewImage>,
) -> impl Responder {
    let car_id = path.into_inner();
    let mut new_img = new_img.into_inner();
    new_img.car_id = car_id;

    let img = match web::block(move || svc.upload(new_img)).await {
        Ok(Ok(c)) => c,
        _ => return HttpResponse::InternalServerError().body("Error listing images"),
    };
    HttpResponse::Created().json(img)
}
