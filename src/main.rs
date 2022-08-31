mod api;
mod model;
mod pubsub;
mod repository;
#[cfg(test)]
mod test;

use crate::api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use crate::pubsub::PubSub;
use crate::repository::mongodb_repo::MongoRepo;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init logger middleware
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let db: MongoRepo = MongoRepo::init().await;
    let db_data: Data<MongoRepo> = Data::new(db);

    let pub_sub = PubSub::init().await;
    let pub_sub_data = Data::new(pub_sub);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(db_data.clone())
            .app_data(pub_sub_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
