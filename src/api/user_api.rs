use crate::model::user_model::User;
use crate::MongoRepo;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;

#[get("/users")]
pub async fn get_all_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => match err {
            Error::InvalidObjectId(_) => {
                HttpResponse::BadRequest().body("User with specified ID not found!")
            }
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        first_name: new_user.first_name.to_owned(),
        last_name: new_user.last_name.to_owned(),
        email: new_user.email.to_owned(),
    };
    let result = db.create_user(data).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = User {
        id: Some(match ObjectId::parse_str(&id) {
            Ok(result) => result,
            Err(_) => return HttpResponse::BadRequest().body("User with specified ID not found!"),
        }),
        first_name: new_user.first_name.to_owned(),
        last_name: new_user.last_name.to_owned(),
        email: new_user.email.to_owned(),
    };
    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(update) => {
            return if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            } else {
                HttpResponse::NotFound().body("No user found with specified ID")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_user(&id).await;
    match result {
        Ok(res) => {
            return if res.deleted_count == 1 {
                HttpResponse::Ok().json("User successfully deleted!")
            } else {
                HttpResponse::NotFound().json("User with specified ID not found!")
            }
        }
        Err(err) => match err {
            Error::InvalidObjectId(_) => {
                HttpResponse::BadRequest().body("User with specified ID not found!")
            }
            _ => HttpResponse::InternalServerError().body(err.to_string()),
        },
    }
}
