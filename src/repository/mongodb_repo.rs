use actix_web::HttpRequest;
use dotenv::dotenv;
use rand::distributions::Alphanumeric;
use rand::{ thread_rng, Rng };

use crate::models::user_model::User;
use mongodb::{
    bson::{ doc, extjson::de::Error, oid::ObjectId },
    results::{ DeleteResult, InsertOneResult, UpdateResult },
    Client,
    Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri: String = "MongoDB Connection String".to_string();
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("Users");
        MongoRepo { col }
    }

    pub async fn log_user(&self, user_info: User) -> Result<User, Error> {
        let filter = doc! { "username": user_info.username, "password": user_info.password };
        let user_detail = self.col.find_one(filter, None).await.ok().expect("Wrong Credentials !");

        Ok(user_detail.unwrap())
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();

        let new_doc = User {
            id: None,
            username: new_user.username,
            password: new_user.password,
            token: Some(token),
        };
        let user = self.col.insert_one(new_doc, None).await.ok().expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String, req: HttpRequest) -> Result<User, Error> {
        let _user_token = req.headers().get("Authorization");
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let user_detail = self.col
            .find_one(filter, None).await
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let new_doc =
            doc! {
            "$set":
                {
                    "id": new_user.id,
                    "username": new_user.username,
                    "passwrod": new_user.password
                },
        };
        let updated_doc = self.col
            .update_one(filter, new_doc, None).await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id":obj_id };
        let user_detail = self.col.delete_one(filter, None).await.ok().expect("Could not delete!");
        Ok(user_detail)
    }
}
