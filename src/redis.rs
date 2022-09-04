use crate::model::setting_model::Settings;
use redis::aio::Connection;
use redis::Client;
use serde::Serialize;
use serde_json::{Error, Value};

#[derive(Debug)]
pub struct RedisClient {
    pub client: Client,
}

impl RedisClient {
    pub async fn init() -> Self {
        let Settings { redis_url, .. } = Settings::from_env();
        let client: Client = Client::open(redis_url).unwrap();

        RedisClient { client }
    }

    pub async fn get_connection(&self) -> Connection {
        let con: Connection = self
            .client
            .get_async_connection()
            .await
            .expect("Get connection failed");

        return con;
    }

    pub async fn set_value<T>(&self, key: &str, data: &T)
    where
        T: ?Sized + Serialize,
    {
        let serialized_data = serde_json::to_string(&data).unwrap();

        redis::cmd("SET")
            .arg(&[key, &serialized_data])
            .query_async::<Connection, String>(&mut self.get_connection().await)
            .await
            .expect("Set value failed");
    }

    pub async fn get_value(&self, key: &str) -> Result<Value, Error> {
        let result: String = redis::cmd("GET")
            .arg(&key)
            .query_async::<Connection, String>(&mut self.get_connection().await)
            .await
            .unwrap_or_default();

        let value = match serde_json::from_str(&result) {
            Ok(val) => val,
            Err(_) => Value::Null,
        };

        Ok(value)
    }

    pub async fn del_value(&self, key: &str) -> Result<(), Error> {
        let _ = redis::cmd("DEL")
            .arg(&key)
            .query_async::<Connection, String>(&mut self.get_connection().await)
            .await;

        Ok(())
    }
}
