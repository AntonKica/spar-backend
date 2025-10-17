use std::env;
use serde::{Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;


#[repr(i32)]
#[derive(Copy, Clone)]
pub enum BusinessProcessType {
    UNKNOWN = -1,
    PRIMARY = 0,
    SUPPORT,
}

#[derive(Serialize)]
pub struct EnumResponse {
    code: i32,
    name: String,
}
impl From<BusinessProcessType> for EnumResponse {
    fn from(value: BusinessProcessType) -> Self {
        match value {
            BusinessProcessType::SUPPORT => {
                EnumResponse {
                    code: BusinessProcessType::SUPPORT as i32,
                    name: "podporný".to_owned()
                }
            }
            BusinessProcessType::PRIMARY => {
                EnumResponse {
                    code: BusinessProcessType::PRIMARY as i32,
                    name: "primárny".to_owned()
                }
            }
            BusinessProcessType::UNKNOWN => {
                EnumResponse {
                    code: BusinessProcessType::UNKNOWN as i32,
                    name: "neznámy".to_owned()
                }
            }
        }
    }
}
impl From<i32> for BusinessProcessType {
    fn from(value: i32) -> Self {
        match value { 
            0 => BusinessProcessType::PRIMARY,
            1 => BusinessProcessType::SUPPORT,
            _ => BusinessProcessType::UNKNOWN
        }
    }
}
#[derive(sqlx::FromRow)]
pub struct BusinessProcessModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub process_type: BusinessProcessType,
    pub responsible: Option<String>,
}

//#[derive()]
pub struct BusinessProcessCreateModel {
    pub name: String,
    pub description: String,
    pub process_type: BusinessProcessType
}

pub struct RoleModel {
    pub code: String,
    pub name: String,
    pub description: String,
}

pub struct RoleCreateModel {
    pub name: String,
    pub description: String,
}

pub struct BusinessProcessRoleCreateModel {
    pub business_process_code: String,
    pub role_code: String,
}

async fn next_code_for(table: &str, code: &str, num_digits: u32, db: &Pool<Postgres>) -> String {
    let prefix = code.to_owned() + "-";
    let default_value = format!("{prefix}{:0width$}", 0, width = num_digits as usize);
    let query = format!("SELECT code FROM {table} ORDER BY code DESC LIMIT 1");

    let top_code: String = sqlx::query_scalar(query.as_str()).fetch_one(db).await.unwrap_or(default_value);
    let top_code_number = top_code
        .strip_prefix(prefix.as_str())
        .unwrap()
        .to_owned()
        .parse::<i32>()
        .unwrap();

    let res = format!("{code}-{:0width$}", top_code_number + 1, width = num_digits as usize);
    res
}
impl BusinessProcessCreateModel {
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<(String), sqlx::Error> {
        let code = next_code_for("business_process", "BP", 4, db).await;
        sqlx::query!(
        r#"INSERT INTO business_process(code, name, description, process_type) VALUES ($1,$2,$3,$4)"#,
        code,
        self.name,
        self.description,
        self.process_type as i32,
        )
            .execute(db)
            .await?;
        Ok(code)
    }
}

pub async fn setResponsible(business_process_code: String, role_code: String, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"UPDATE business_process SET responsible = $2 WHERE code = $1"#,
        business_process_code,
        role_code,
        )
        .execute(db)
        .await?;
    Ok(())
}
impl BusinessProcessRoleCreateModel {
    pub async fn assign(&self, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__role(business_process_code, role_code) VALUES ($1,$2)"#,
        self.business_process_code,
        self.role_code,
        )
            .execute(db)
            .await?;
        Ok(())
    }
}

impl RoleCreateModel {
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<(String), sqlx::Error> {
        let code = next_code_for("role", "RL", 4, db).await;
        sqlx::query!(
        r#"INSERT INTO role(code, name, description) VALUES ($1,$2,$3)"#,
        code,
        self.name,
        self.description,
        )
            .execute(db)
            .await?;
        Ok((code))
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
