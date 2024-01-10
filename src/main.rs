mod api;
mod models;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::user_api::{log_user ,create_user, delete_user, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .service(log_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
            .service(get_user)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
