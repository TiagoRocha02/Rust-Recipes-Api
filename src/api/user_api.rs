use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};

use mongodb::bson::oid::ObjectId;

#[post("/user")]
pub async fn log_user(db: Data<MongoRepo>, user_info: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        username: user_info.username.to_owned(),
        password: user_info.password.to_owned(),
        token: None,
    };
    let user_detail = db.log_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("/user/create")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        password: new_user.password.to_owned(),
        token: None,
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>, req: HttpRequest) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let user_detail = db.get_user(&id, req).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_user: Json<User>,
    req: HttpRequest,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        username: new_user.username.to_owned(),
        password: new_user.password.to_owned(),
        token: None,
    };
    let update_result = db.update_user(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id, req).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid Id");
    }
    let delete_result = db.delete_user(&id).await;
    match delete_result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User deleted!");
            } else {
                HttpResponse::NotFound().json("User with that id no found!")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
