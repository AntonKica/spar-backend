use actix_web::{HttpResponse, ResponseError};
use chrono::{NaiveDate, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{PgConnection};
use crate::enums::{BusinessProcessType, ModuleType};
use crate::service::{ApiError, ApiResult};

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

#[derive(Deserialize, Clone)]
pub struct TOURElementaryThreatUpdateModel {
    pub elementary_threat_code: String,
    pub relevance: i32,
    pub comment: String,
    pub reviewed: bool
}