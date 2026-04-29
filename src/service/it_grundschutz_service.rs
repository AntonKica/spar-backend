use crate::enums::ThreatCategory;
use crate::model::it_grundchutz_models::{ItGrundschutzModule, ItGrundschutzModuleRequirement, ThreatModel};
use crate::service::ApiResult;
use actix_web::{get, web, web::Json, web::Path};
use sqlx::{Pool, Postgres};

pub struct ItGrundschutzService;

impl ItGrundschutzService {
    pub async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<ItGrundschutzModule>> {
        let rows = sqlx::query_as!(
            ItGrundschutzModule,
            r#"
            SELECT code, name, description
            FROM it_grundschutz_module
            ORDER BY code
            "#
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }
    pub async fn threats_by_module(
        db: &Pool<Postgres>,
        module_code: String,
    ) -> ApiResult<Vec<ThreatModel>> {
        let rows = sqlx::query_as!(
            ThreatModel,
            r#"
            SELECT
                t.code,
                t.name,
                t.description,
                t.confidentiality_impaired,
                t.integrity_impaired,
                t.availability_impaired,
                t.category AS "category!: ThreatCategory"
            FROM threat t
            JOIN it_grundschutz_module_threat mt ON mt.threat = t.code
            WHERE mt.it_grundschutz_module = $1
            "#,
            module_code,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }
    pub async fn requirements_by_module(
        db: &Pool<Postgres>,
        module_code: String,
    ) -> ApiResult<Vec<ItGrundschutzModuleRequirement>> {
        let rows = sqlx::query_as!(
            ItGrundschutzModuleRequirement,
            r#"
            SELECT code, module, description
            FROM it_grundschutz_module_requirement
            WHERE module = $1
            ORDER BY code
            "#,
            module_code,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }
}