use actix_web::{HttpResponse, ResponseError};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{PgConnection};
use thiserror::Error;
use crate::enums::{BusinessProcessType, ModuleType};
use crate::service::service::ApiError;
use crate::workflow::{create_risk_analysis_process_workflow, Workflow};
use crate::workflow_model::create_workflow_model;

#[derive(Debug, Clone)]
pub struct BusinessProcessModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub process_type: BusinessProcessType,
    pub responsible: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BusinessProcessCreateModel {
    pub name: String,
    pub description: String,
    pub process_type: BusinessProcessType
}

#[derive(Debug, Clone)]
pub struct RoleModel {
    pub code: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct RoleCreateModel {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct BusinessProcessRoleCreateModel {
    pub business_process_code: String,
    pub role_code: String,
}

#[derive(Debug, Clone)]
pub struct ApplicationModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub application_user: String,
    pub responsible: String,
}
#[derive(Debug, Clone)]
pub struct ApplicationCreateModel {
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub application_user: String,
    pub responsible: String,
}

#[derive(Debug, Clone)]
pub struct BusinessProcessApplicationModel {
    pub business_process_code: String,
    pub application_code: String,
}
#[derive(Debug, Clone)]
pub struct BusinessProcessApplicationCreateModel {
    pub business_process_code: String,
    pub application_code: String,
}

#[derive(Debug, Clone)]
pub struct ITSystemModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub count: i32,
    pub application_user: String,
    pub responsible: String,
}

#[derive(Debug, Clone)]
pub struct ITSystemCreateModel {
    pub name: String,
    pub description: String,
    pub module_type: ModuleType,
    pub count: i32,
    pub application_user: String,
    pub responsible: String,
}

#[derive(Debug, Clone)]
pub struct AssetCreateModel {
    pub name: String,
    pub description: String,
    pub responsible: String,
}
#[derive(Serialize)]
pub struct AssetModel {
    pub code: String,
    pub name: String,
    pub description: String,
    pub responsible: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RiskAnalysisProcessCreateModel {
    pub target_objects_under_review: Vec<String>
}

#[derive(Debug, Clone)]
pub struct TargetObjectUnderReviewCreateModel {
    pub risk_analysis_process_code: String,
    pub asset_code: String,
}

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Code generation error: {0}")]
    CodeGeneration(String),

    #[error("Invalid code format: {0}")]
    InvalidCodeFormat(String),
}

pub type ModelResult<T> = Result<T, ModelError>;

pub trait Creatable: Sized {
    const TABLE_NAME: &'static str;
    const CODE_PREFIX: &'static str;
    const CODE_DIGITS: usize;

    async fn create(&self, tx: &mut PgConnection) -> ModelResult<String>;
}

pub async fn next_code_for(
    table: &str,
    acronym: &str,
    code_length: usize,
    tx: &mut PgConnection
) -> ModelResult<String> {
    // TOTAL LENGTH - STRING - '-'
    let prefix = format!("{acronym}-");
    let number_length = code_length - prefix.len();

    let query = format!("SELECT code FROM {table} ORDER BY code DESC LIMIT 1");
    let top_code: Option<String> = sqlx::query_scalar(&query).fetch_optional(tx).await?;
    let next_number = match top_code {
        Some(code) => {
            code.strip_prefix(&prefix)
                .ok_or(ModelError::InvalidCodeFormat(format!("Invalid code format {}", code.to_owned())))?
                .parse::<usize>()
                .map_err(|e| ModelError::InvalidCodeFormat(format!("Invalid code number {e}")))?
            + 1
        }
        None => 1
    };

    Ok(format!("{prefix}{next_number:0number_length$}"))
}

impl Creatable for BusinessProcessCreateModel {
    const TABLE_NAME: &'static str = "business_process";
    const CODE_PREFIX: &'static str = "BP";
    const CODE_DIGITS: usize = 7;

    async fn create(
        &self,
        tx: &mut PgConnection
    ) -> ModelResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query!(
        r#"INSERT INTO business_process(code, name, description, process_type) VALUES ($1,$2,$3,$4)"#,
        code,
        self.name,
        self.description,
        self.process_type as i32,
        )
            .execute(tx)
            .await?;

        Ok(code)
    }
}
impl BusinessProcessRoleCreateModel {
    pub async fn assign(
        &self,
        tx: &mut PgConnection) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__role(business_process_code, role_code) VALUES ($1,$2)"#,
        self.business_process_code,
        self.role_code,
        )
            .execute(tx)
            .await?;
        Ok(())
    }
}

impl Creatable for RoleCreateModel {
    const TABLE_NAME: &'static str = "role";
    const CODE_PREFIX: &'static str = "RL";
    const CODE_DIGITS: usize = 7;
    async fn create(
        &self,
        tx: &mut PgConnection
    ) -> ModelResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;
        sqlx::query!(
        r#"INSERT INTO role(code, name, description) VALUES ($1,$2,$3)"#,
        code,
        self.name,
        self.description,
        )
            .execute(tx)
            .await?;
        Ok(code)
    }
}

impl Creatable for ApplicationCreateModel {
    const TABLE_NAME: &'static str = "application";
    const CODE_PREFIX: &'static str = "APP";
    const CODE_DIGITS: usize = 9;
    async fn create(
        &self,
        tx: &mut PgConnection
    ) -> ModelResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;
        sqlx::query!(
        r#"INSERT INTO application(code, name, description, module_type, responsible, application_user) VALUES ($1,$2,$3,$4,$5,$6)"#,
        code,
        self.name,
        self.description,
        self.module_type as i32,
            self.responsible,
            self.application_user
        )
            .execute(tx)
            .await?;
        Ok(code)
    }
}

impl Creatable for ITSystemCreateModel {
    const TABLE_NAME: &'static str = "it_system";
    const CODE_PREFIX: &'static str = "ITS";
    const CODE_DIGITS: usize = 9;
    async fn create(
        &self,
        tx: &mut PgConnection
    ) -> ModelResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;
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
            .execute(tx)
            .await?;
        Ok(code)
    }
}

impl BusinessProcessApplicationCreateModel {
    pub async fn assign(
        &self,
        tx: &mut PgConnection
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"INSERT INTO business_process__application(business_process_code, application_code) VALUES ($1,$2)"#,

        self.business_process_code,
        self.application_code,
        )
            .execute(tx)
            .await?;
        Ok(())
    }
}

pub async fn set_responsible(
    business_process_code: String,
    role_code: String,
    tx: &mut PgConnection
) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"UPDATE business_process SET responsible = $2 WHERE code = $1"#,
        business_process_code,
        role_code,
        )
        .execute(tx)
        .await?;
    Ok(())
}

impl Creatable for AssetCreateModel {
    const TABLE_NAME: &'static str = "asset";
    const CODE_PREFIX: &'static str = "AST";
    const CODE_DIGITS: usize = 10;
    async fn create(
        &self,
        tx: &mut PgConnection
    ) -> ModelResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;
        sqlx::query!(
        r#"INSERT INTO asset(code, name, description, responsible) VALUES ($1,$2,$3,$4)"#,
        code,
        self.name,
        self.description,
            self.responsible,
        )
            .execute(tx)
            .await?;
        Ok(code)
    }
}