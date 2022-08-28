use crate::model::user_model::User;
use actix_web::{
    test::{call_and_read_body, init_service, TestRequest},
    web::Bytes,
};

use super::*;

#[actix_web::test]
async fn test() {
    let db: MongoRepo = MongoRepo::init().await;
    let db_data: Data<MongoRepo> = Data::new(db);

    let app = init_service(
        App::new()
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(get_all_users)
    )
    .await;

    let user = User {
        id: None,
        first_name: "Jane".into(),
        last_name: "Doe".into(),
        email: "example@example.com".into(),
    };

    let req = TestRequest::post()
        .uri("/user")
        .set_json(&user)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"user added"));
}
