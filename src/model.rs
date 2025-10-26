use sqlx::{Pool, Postgres};
use crate::enums::{BusinessProcessType, ModuleType};
use crate::response::EnumResponse;

pub struct BusinessProcessModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub process_type: BusinessProcessType,
    pub responsible: Option<String>,
}

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

pub struct ApplicationModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub application_user: String,
    pub responsible: String,
}
pub struct ApplicationCreateModel {
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub application_user: String,
    pub responsible: String,
}

pub struct BusinessProcessApplicationModel {
    pub business_process_code: String,
    pub application_code: String,
}
pub struct BusinessProcessApplicationCreateModel {
    pub business_process_code: String,
    pub application_code: String,
}

pub struct ITSystemModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub count: i32,
    pub application_user: String,
    pub responsible: String,
}

pub struct ITSystemCreateModel {
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub count: i32,
    pub application_user: String,
    pub responsible: String,
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
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<String, sqlx::Error> {
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
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<String, sqlx::Error> {
        let code = next_code_for("role", "RL", 4, db).await;
        sqlx::query!(
        r#"INSERT INTO role(code, name, description) VALUES ($1,$2,$3)"#,
        code,
        self.name,
        self.description,
        )
            .execute(db)
            .await?;
        Ok(code)
    }
}

impl ApplicationCreateModel {
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<String, sqlx::Error> {
        let code = next_code_for("application", "APP", 5, db).await;
        sqlx::query!(
        r#"INSERT INTO application(code, name, description, module_type, responsible, application_user) VALUES ($1,$2,$3,$4,$5,$6)"#,
        code,
        self.name,
        self.description,
        self.module_type as i32,
            self.responsible,
            self.application_user
        )
            .execute(db)
            .await?;
        Ok(code)
    }
}

impl ITSystemCreateModel {
    pub async fn create(&self, db: &Pool<Postgres>) -> Result<String, sqlx::Error> {
        let code = next_code_for("it_system", "ITS", 5, db).await;
        sqlx::query!(
        r#"INSERT INTO it_system(code, name, description, module_type, count, responsible, application_user) VALUES ($1,$2,$3,$4,$5,$6,$7)"#,
        code,
        self.name,
        self.description,
        self.module_type as i32,
            self.count,
            self.responsible,
            self.application_user
        )
            .execute(db)
            .await?;
        Ok(code)
    }
}

impl BusinessProcessApplicationCreateModel {
    pub async fn assign(&self, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__application(business_process_code, application_code) VALUES ($1,$2)"#,
        self.business_process_code,
        self.application_code,
        )
            .execute(db)
            .await?;
        Ok(())
    }
}

pub async fn set_responsible(business_process_code: String, role_code: String, db: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"UPDATE business_process SET responsible = $2 WHERE code = $1"#,
        business_process_code,
        role_code,
        )
        .execute(db)
        .await?;
    Ok(())
}
