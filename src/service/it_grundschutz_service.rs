use crate::enums::ThreatCategory;
use crate::model::it_grundchutz_models::ThreatModel;
use crate::service::ApiResult;
use actix_web::{get, web, web::Json, web::Path};
use sqlx::{Pool, Postgres};

pub struct ItGrundschutzService;

impl ItGrundschutzService {
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
}