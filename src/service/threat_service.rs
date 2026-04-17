use sqlx::{PgConnection, Pool, Postgres};
use crate::enums::ThreatCategory;
use crate::model::it_grundchutz_models::{ItGrundschutzModule, ThreatModel};
use crate::service::{ApiError, ApiResult, GeneralService};

#[derive(Debug, Clone, serde::Deserialize, utoipa::ToSchema)]
pub struct ThreatModelCreate {
    pub name: String,
    pub description: String,
    pub confidentiality_impaired: bool,
    pub integrity_impaired: bool,
    pub availability_impaired: bool,
    pub category: ThreatCategory,
}

pub struct ThreatService;

impl GeneralService<ThreatModelCreate, ThreatModel, ThreatModel> for ThreatService {
    async fn create(
        tx: &mut PgConnection,
        model_create: ThreatModelCreate,
    ) -> ApiResult<String> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO threat (
                code,
                name,
                description,
                confidentiality_impaired,
                integrity_impaired,
                availability_impaired,
                category
            )
            VALUES (
                'THR-' || LPAD(nextval('specific_threat_code_seq')::TEXT, 4, '0'),
                $1, $2, $3, $4, $5, $6
            )
            RETURNING code
            "#,
            model_create.name,
            model_create.description,
            model_create.confidentiality_impaired,
            model_create.integrity_impaired,
            model_create.availability_impaired,
            model_create.category as ThreatCategory,
        )
            .fetch_one(tx)
            .await?;

        Ok(rec.code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<ThreatModel>> {
        let rows = sqlx::query_as!(
            ThreatModel,
            r#"
            SELECT
                code,
                name,
                description,
                confidentiality_impaired,
                integrity_impaired,
                availability_impaired,
                category AS "category!: ThreatCategory"
            FROM threat
            ORDER BY code
            "#
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    async fn detail(
        db: &Pool<Postgres>,
        code: String,
    ) -> ApiResult<Option<ThreatModel>> {
        let threat = sqlx::query_as!(
            ThreatModel,
            r#"
            SELECT
                code,
                name,
                description,
                confidentiality_impaired,
                integrity_impaired,
                availability_impaired,
                category AS "category!: ThreatCategory"
            FROM threat
            WHERE code = $1
            "#,
            code,
        )
            .fetch_optional(db)
            .await?;

        Ok(threat)
    }
}

impl ThreatService {
    pub async fn delete(tx: &mut PgConnection, code: String) -> ApiResult<()> {
        if code.starts_with("G-") {
            return Err(ApiError::Validation(format!(
                "Cannot delete elementary threat {code}"
            )));
        }

        let result = sqlx::query!(
            r#"DELETE FROM threat WHERE code = $1"#,
            code,
        )
            .execute(tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(format!("Threat {code} not found")));
        }

        Ok(())
    }
}