use std::path::PathBuf;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};
use actix_web::Result;
use actix_web::error::ErrorInternalServerError;
use crate::{
    models::NewImage,
    services::image_service::ImageService,
};

/// GET /cars/{car_id}/images
pub async fn list_images(
    svc: web::Data<ImageService>,
    path: web::Path<i64>,
) -> impl Responder {
    let car_id = path.into_inner();

    let imgs = match web::block(move || svc.list_for_car(car_id)).await {
        Ok(Ok(c)) => c,
        Ok(Err(e)) => {
            return HttpResponse::InternalServerError().body(format!("Error listing images: {}", e));
        }
        Err(block_err) => {
            log::error!("blocking error: {:?}", block_err);
            return HttpResponse::InternalServerError()
                .body(format!("Error listing images: {}", block_err));
        }
    };
    HttpResponse::Ok().json(imgs)
}

/// GET /cars/{car_id}/image
pub async fn get_image_by_id(
    svc: web::Data<ImageService>,
    path: web::Path<i64>,
) -> Result<NamedFile> {
    let img_id = path.into_inner();

    let img = web::block(move || svc.get(img_id))
        .await
        .map_err(ErrorInternalServerError)?       // BlockingError → actix Error
        .map_err(ErrorInternalServerError)?;      // repo Error    → actix Error

    let fname = img.url.rsplit('/').next().unwrap_or(&img.url);
    let disk: PathBuf = ["uploads", &img.car_id.to_string(), fname].iter().collect();

    Ok(NamedFile::open(disk)?)
}

/// POST /cars/{car_id}/image
pub async fn upload_image(
    svc: web::Data<ImageService>,
    path: web::Path<i64>,
    mut payload: Multipart,
) -> impl Responder {
    use actix_web::web::BytesMut;
    use chrono::Utc;
    use futures_util::{StreamExt, TryStreamExt};
    let car_id = path.into_inner();

    let mut buf = BytesMut::new();
    let mut field = match payload.try_next().await {
        Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        Ok(None) => return HttpResponse::BadRequest().body("no file field"),
        Ok(Some(f)) => f,
    };

    while let Some(chunk_res) = field.next().await {
        match chunk_res {
            Ok(chunk) => buf.extend_from_slice(&chunk),
            Err(e) => return HttpResponse::BadRequest().body(e.to_string()),
        }
    }

    let dir = format!("uploads/{}", car_id);
    if let Err(e) = tokio::fs::create_dir_all(&dir).await {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let fname = format!("{}.jpg", Utc::now().timestamp_millis());
    let disk_path = format!("{}/{}", dir, fname);
    if let Err(e) = tokio::fs::write(&disk_path, &buf[..]).await {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let new_img = NewImage {
        car_id,
        url: format!("/cars/{}/image/{}", car_id, fname),
    };

    match web::block(move || svc.upload(new_img)).await {
        Ok(Ok(img)) => HttpResponse::Created().json(img),
        Ok(Err(e)) => HttpResponse::InternalServerError().body(e.to_string()),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
