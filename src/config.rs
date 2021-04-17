use anyhow::Result;
use sqlx::PgPool;

pub struct Config {
    pub address: String,
    pub db_address: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        Self {
            address: dotenv::var("ADDRESS").unwrap_or(String::from("localhost:8080")),
            db_address: dotenv::var("DATABASE_URL").unwrap_or(String::from(
                "DATABASE_URL=postgres://admin:admin@localhost:5666/coomoji",
            )),
        }
    }
}
