use actix_web::{web, App, HttpServer};
use salon_api::repositories::car_repository::CarRepository;
use salon_api::repositories::image_repository::ImageRepository;
use salon_api::repositories::user_repository::UserRepository;
use salon_api::services::car_service::CarService;
use salon_api::services::image_service::ImageService;
use salon_api::services::user_service::UserService;
use salon_api::{
    auth::jwt::JwtManager,
    auth::middleware::auth_middleware,
    config::jwt_config::AuthConfig,
    db::establish_pool,
    handlers::{auth_handlers, car_handlers, image_handlers},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load config & shared state
    env_logger::init();
    let auth_cfg = AuthConfig::from_env();
    let jwt_mgr = JwtManager::new(&auth_cfg);
    let pool = establish_pool();

    // build repos & services
    let car_svc = CarService::new(CarRepository::new(pool.clone()));
    let user_svc = UserService::new(UserRepository::new(pool.clone()));
    let image_svc = ImageService::new(ImageRepository::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            // shared data
            .app_data(web::Data::new(jwt_mgr.clone()))
            .app_data(web::Data::new(auth_cfg.clone()))
            .app_data(web::Data::new(car_svc.clone()))
            .app_data(web::Data::new(user_svc.clone()))
            .app_data(web::Data::new(image_svc.clone()))
            // public auth routes
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth_handlers::register))
                    .route("/login", web::post().to(auth_handlers::login)),
            )
            // protected routes
            .service(
                web::scope("/cars")
                    .wrap(auth_middleware())
                    .route("", web::get().to(car_handlers::list_cars))
                    .route("", web::post().to(car_handlers::create_car))
                    .route("/{id}", web::get().to(car_handlers::get_car))
                    .service(
                        web::scope("/{car_id}/images")
                            .route("", web::get().to(image_handlers::list_images))
                    )
                    .service(
                        web::scope("/{car_id}/image") // single image
                            .route("", web::post().to(image_handlers::upload_image)),
                    )
            )
            .service(
                web::scope("/image")
                    .wrap(auth_middleware())
                    .route("/{id}", web::get().to(image_handlers::get_image_by_id))
            )

    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
