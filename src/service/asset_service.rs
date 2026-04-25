use crate::enums::ProtectionRequirement;
use crate::model::it_grundchutz_models::ItGrundschutzModule;
use sqlx::{PgConnection, Pool, Postgres};
use crate::service::{ApiError, ApiResult, GeneralService};
use crate::model::asset_model::{AssetCreateModel, AssetDetailModel, AssetModel};

pub struct AssetService;

impl GeneralService<AssetCreateModel, AssetModel, AssetDetailModel> for AssetService {
    async fn create(
        tx: &mut PgConnection,
        model_create: AssetCreateModel,
    ) -> ApiResult<String> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO asset (
                name, description, module,
                confidentiality_protection_requirement,
                integrity_protection_requirement,
                availability_protection_requirement,
                confidentiality_protection_requirement_description,
                integrity_protection_requirement_description,
                availability_protection_requirement_description
            )
            VALUES (
                $1, $2, $3,
                $4::protection_requirement,
                $5::protection_requirement,
                $6::protection_requirement,
                $7, $8, $9
            )
            RETURNING code
            "#,
            model_create.name,
            model_create.description,
            model_create.module,
            model_create.confidentiality_protection_requirement as ProtectionRequirement,
            model_create.integrity_protection_requirement as ProtectionRequirement,
            model_create.availability_protection_requirement as ProtectionRequirement,
            model_create.confidentiality_protection_requirement_description,
            model_create.integrity_protection_requirement_description,
            model_create.availability_protection_requirement_description,
        )
            .fetch_one(tx)
            .await?;

        Ok(rec.code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<AssetModel>> {
        let rows = sqlx::query_as!(
            AssetModel,
            r#"
            SELECT
                code,
                name,
                description,
                module,
                confidentiality_protection_requirement AS "confidentiality_protection_requirement!: ProtectionRequirement",
                integrity_protection_requirement AS "integrity_protection_requirement!: ProtectionRequirement",
                availability_protection_requirement AS "availability_protection_requirement!: ProtectionRequirement",
                confidentiality_protection_requirement_description,
                integrity_protection_requirement_description,
                availability_protection_requirement_description
            FROM asset
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
    ) -> ApiResult<Option<AssetDetailModel>> {
        let asset = sqlx::query_as!(
            AssetModel,
            r#"
            SELECT
                code,
                name,
                description,
                module,
                confidentiality_protection_requirement AS "confidentiality_protection_requirement!: ProtectionRequirement",
                integrity_protection_requirement AS "integrity_protection_requirement!: ProtectionRequirement",
                availability_protection_requirement AS "availability_protection_requirement!: ProtectionRequirement",
                confidentiality_protection_requirement_description,
                integrity_protection_requirement_description,
                availability_protection_requirement_description
            FROM asset
            WHERE code = $1
            "#,
            code,
        )
            .fetch_optional(db)
            .await?;

        let Some(asset) = asset else {
            return Ok(None);
        };

        let module = sqlx::query_as!(
            ItGrundschutzModule,
            r#"
            SELECT code, name, description
            FROM it_grundschutz_module
            WHERE code = $1
            "#,
            asset.module,
        )
            .fetch_one(db)
            .await?;

        Ok(Some(AssetDetailModel {
            code: asset.code,
            name: asset.name,
            description: asset.description,
            module,
            confidentiality_protection_requirement: asset.confidentiality_protection_requirement,
            integrity_protection_requirement: asset.integrity_protection_requirement,
            availability_protection_requirement: asset.availability_protection_requirement,
            confidentiality_protection_requirement_description: asset.confidentiality_protection_requirement_description,
            integrity_protection_requirement_description: asset.integrity_protection_requirement_description,
            availability_protection_requirement_description: asset.availability_protection_requirement_description,
        }))
    }
}
impl AssetService {
    pub async fn distinct_modules(db: &Pool<Postgres>) -> ApiResult<Vec<ItGrundschutzModule>> {
        let rows = sqlx::query_as!(
            ItGrundschutzModule,
            r#"
            SELECT DISTINCT m.code, m.name, m.description
            FROM it_grundschutz_module m
            JOIN asset a ON a.module = m.code
            ORDER BY m.code
            "#
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }
}