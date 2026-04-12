use crate::model::it_grundchutz_models::ItGrundschutzThreat;
use crate::service::ApiResult;
use actix_web::{get, web, web::Json, web::Path};
use sqlx::{Pool, Postgres};

pub struct ItGrundschutzService;

impl ItGrundschutzService {
    pub async fn threats_by_module(
        db: &Pool<Postgres>,
        module_code: String,
    ) -> ApiResult<Vec<ItGrundschutzThreat>> {
        let rows = sqlx::query_as!(
            ItGrundschutzThreat,
            r#"
            SELECT
                t.code,
                t.name,
                t.description,
                t.confidentiality_impaired,
                t.integrity_impaired,
                t.availability_impaired
            FROM it_grundschutz_threat t
            JOIN it_grundschutz_module_threat mt ON mt.it_grundschutz_threat = t.code
            WHERE mt.it_grundschutz_module = $1
            "#,
            module_code,
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }
}