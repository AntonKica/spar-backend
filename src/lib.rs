use std::env;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct BusinessProcessModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub process_type: String,
    pub responsible: String,
}

pub struct StaffModel {
    pub code: String,
    pub name: String,
    pub description: String,
}

pub struct BusinessProcessStaffModel {
    pub business_process_code: String,
    pub staff_code: String,
}

impl BusinessProcessModel {
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query!(
        r#"INSERT INTO business_process(code, name, description, process_type, responsible) VALUES ($1,$2,$3,$4,$5)"#,
        self.code,
        self.name,
        self.description,
        self.process_type,
        self.responsible,
        )
            .execute(db)
            .await?;
        Ok(())
    }

    pub async fn assign_staff(&self, staff_model: &StaffModel, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__staff(business_process_code, staff_code) VALUES ($1,$2)"#,
        self.code,
        staff_model.code,
        )
            .execute(db)
            .await?;
        Ok(())
    }
}

impl StaffModel {
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query!(
        r#"INSERT INTO staff(code, name, description) VALUES ($1,$2,$3)"#,
        self.code,
        self.name,
        self.description,
        )
            .execute(db)
            .await?;
        Ok(())
    }
}

pub async fn create_connection() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
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
