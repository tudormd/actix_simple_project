use crate::model::setting_model::Settings;
use crate::model::user_model::User;
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind, Result};

#[derive(Debug)]
pub struct PubSub {
    conn: Connection,
}

impl PubSub {
    pub async fn init() -> Self {
        let Settings { amqp_addr, .. } = Settings::from_env();
        let conn = Connection::connect(&amqp_addr, ConnectionProperties::default())
            .await
            .unwrap();

        PubSub { conn }
    }

    pub async fn create_channel(&self) -> Result<Channel> {
        self.conn.create_channel().await
    }

    pub async fn publish_user_created(&self, user: &User) -> Result<()> {
        let chanel = self.create_channel().await?;
        let exchange = "rust-user";
        let queue = "rust-user-created";
        let routing_key = "rust-user.created";
        chanel
            .exchange_declare(
                exchange,
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        chanel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await?;

        chanel
            .queue_bind(
                queue,
                exchange,
                routing_key,
                Default::default(),
                Default::default(),
            )
            .await?;

        let payload: Vec<u8> = bincode::serialize(&user).unwrap();

        chanel
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                &*payload,
                BasicProperties::default(),
            )
            .await?;

        Ok(())
    }
}
