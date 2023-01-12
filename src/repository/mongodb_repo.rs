use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult,
    Client, Collection,
};

// use crate::models::user_model::User;

use crate::models::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("MONGOURI must be set");
        let client = Client::with_uri_str(uri)
            .await
            .expect("Error connecting to MongoDB");
        let db = client.database("zero2prod");
        let col: Collection<User> = db.collection("Users");
        MongoRepo { col }
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating user");
        Ok(user)
    }
}
