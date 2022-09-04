use crate::model::setting_model::Settings;
use crate::model::user_model::User;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{
    bson::extjson::de::Error, options::ClientOptions, results::InsertOneResult, Client, Collection,
    Database,
};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";

pub struct MongoRepo {
    user_collection: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        // Read the application settings from the env.
        let Settings { database_url, .. } = Settings::from_env();
        // Create the database connection for the application.
        let options: ClientOptions = ClientOptions::parse(database_url).await.unwrap();
        let client: Client = Client::with_options(options).unwrap();
        let db: Database = client.database(DB_NAME);
        let user_collection: Collection<User> = db.collection::<User>(COLL_NAME);

        MongoRepo { user_collection }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            email: new_user.email,
        };
        let user = self
            .user_collection
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(result) => result,
            Err(e) => return Err(Error::from(e)),
        };
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_collection
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(result) => result,
            Err(e) => return Err(Error::from(e)),
        };
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "first_name": new_user.first_name,
                    "last_name": new_user.last_name,
                    "email": new_user.email
                },
        };
        let updated_doc = self
            .user_collection
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = match ObjectId::parse_str(id) {
            Ok(result) => result,
            Err(e) => return Err(Error::from(e)),
        };
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .user_collection
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
}
