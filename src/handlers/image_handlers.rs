use std::path::{Path};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult, error};
use futures_util::{StreamExt, TryStreamExt};
use crate::{
    models::NewImage,
    services::image_service::ImageService,
};
use crate::auth::guards::require_admin;

pub(crate) const IMAGES_ROOT: &str = "/usr/local/bin/uploads/";

/// GET /cars/{car_id}/images
pub async fn list_images(
    svc: web::Data<ImageService>,
    path: web::Path<i64>,
) -> ActixResult<HttpResponse> {
    let car_id = path.into_inner();

    let imgs = web::block(move || svc.list_for_car(car_id))
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(imgs))
}

/// GET /image/{id}
pub async fn get_image_by_id(
    svc: web::Data<ImageService>,
    path: web::Path<i64>,
) -> ActixResult<NamedFile> {
    let img_id = path.into_inner();

    let img = web::block(move || svc.get(img_id))
        .await
        .map_err(error::ErrorInternalServerError)? 
        .map_err(error::ErrorInternalServerError)?;  

    let fname = img.url.rsplit('/').next().unwrap_or(&img.url);
    let disk = Path::new(IMAGES_ROOT)
        .join(img.car_id.to_string())
        .join(fname);

    NamedFile::open(&disk)
        .map_err(|e| {
            log::error!("Could not open {}: {}", disk.display(), e);
            error::ErrorNotFound("Image not found")
        })
}

/// POST /cars/{car_id}/image
pub async fn upload_image(
    req: HttpRequest,
    svc: web::Data<ImageService>,
    path: web::Path<i64>,
    mut payload: Multipart,
) -> ActixResult<HttpResponse> {
    require_admin(&req)?;

    let car_id = path.into_inner();
    let mut buf = web::BytesMut::new();
    let mut field = payload
        .try_next()
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()))?
        .ok_or_else(|| error::ErrorBadRequest("no file field"))?;

    while let Some(chunk) = field.next().await {
        let chunk = chunk.map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        buf.extend_from_slice(&chunk);
    }
    
    let dir = format!("{}{}", IMAGES_ROOT, car_id);
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let fname = format!("{}.jpg", chrono::Utc::now().timestamp_millis());
    let disk_path = format!("{}/{}", &dir, &fname);
    tokio::fs::write(&disk_path, &buf)
        .await
        .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

    let new_img = NewImage {
        car_id,
        url: format!("{}{}/{}", IMAGES_ROOT, car_id, fname),
    };
    let img = web::block(move || svc.upload(new_img))
        .await
        .map_err(error::ErrorInternalServerError)?       
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(img))
}

/// DELETE /image/{id}
pub async fn delete_image(
    req: HttpRequest,
    svc: web::Data<ImageService>,
    path: web::Path<i64>,
) -> ActixResult<HttpResponse> {
    require_admin(&req)?;
    let id = path.into_inner();

    // 1 ─ fetch the record so we know the filename
    let img = web::block({
        let svc = svc.clone();
        move || svc.get(id)
    })
        .await
        .map_err(error::ErrorInternalServerError)?
        .map_err(error::ErrorInternalServerError)?;

    let fname = img.url.rsplit('/').next().unwrap_or("");
    let disk_path = format!("{}{}/{}", IMAGES_ROOT, img.car_id, fname);

    tokio::fs::remove_file(&disk_path)
        .await
        .map_err(|e| {
            log::error!("file delete failed: {}", e);
            error::ErrorInternalServerError("failed to delete image file")
        })?;

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
