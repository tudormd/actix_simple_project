use std::env;

/// The settings to run an application instance.
pub struct Settings {
    /// The web server port to run on.
    pub port: u16,
    /// The MongoDB database url.
    pub database_url: String,
    pub amqp_addr: String,
    pub redis_url: String,
}

impl Settings {
    /// Create an instance of the Settings struct
    /// from the environment variables.
    pub fn from_env() -> Self {
        let port: u16 = env::var("PORT")
            .expect("PORT expected")
            .parse()
            .expect("PORT must be a number");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL expected");
        let amqp_addr = env::var("AMQP_ADDR").expect("AMQP_ADDR expected");
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL expected");
        Self {
            port,
            database_url,
            amqp_addr,
            redis_url,
        }
    }
}
