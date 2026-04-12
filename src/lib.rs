use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::configuration::AppConfig;

pub mod configuration;
pub mod route;
pub mod service;
pub mod api;
pub mod model;
/*
pub mod response;
pub mod enums;

 */

pub async fn create_connection(app_config: &AppConfig) -> Pool<Postgres> {
    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&app_config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}
