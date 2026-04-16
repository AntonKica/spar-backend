use crate::model::it_grundchutz_models::ItGrundschutzModule;
use sqlx::{PgConnection, Pool, Postgres};
use crate::service::{ApiError, ApiResult, GeneralService};
use crate::model::asset_model::{AssetModel, AssetModelCreate, AssetModelDetail};

pub struct AssetService;

impl GeneralService<AssetModelCreate, AssetModel, AssetModelDetail> for AssetService {
    async fn create(tx: &mut PgConnection, model_create: AssetModelCreate) -> ApiResult<String> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO asset (name, description, module)
            VALUES ($1, $2, $3)
            RETURNING code
            "#,
            model_create.name,
            model_create.description,
            model_create.module,
        )
            .fetch_one(tx)
            .await?;

        Ok(rec.code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<AssetModel>> {
        let rows = sqlx::query_as!(
            AssetModel,
            r#"
            SELECT code, name, description, module
            FROM asset
            "#
        )
            .fetch_all(db)
            .await?;

        Ok(rows)
    }

    async fn detail(db: &Pool<Postgres>, code: String) -> ApiResult<Option<AssetModelDetail>> {
        let asset = sqlx::query_as!(
        AssetModel,
        r#"
        SELECT code, name, description, module
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

        Ok(Some(AssetModelDetail {
            code: asset.code,
            name: asset.name,
            description: asset.description,
            module,
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

/*
impl GeneralService<AssetModel, AssetDetailModel, AssetCreateModel> for AssetService {
    const TABLE_NAME: &'static str = "asset";
    const CODE_PREFIX: &'static str = "AST";
    const CODE_DIGITS: usize = 10;

    async fn create(
        tx: &mut PgConnection,
        create_model: AssetCreateModel,
    ) -> ApiResult<String> {
        let code = next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, tx).await?;

        sqlx::query(r#"INSERT INTO asset VALUES ($1,$2,$3,$4,$5,$6,$7)"#)
            .bind(code.clone())
            .bind(create_model.name)
            .bind(create_model.asset_type)
            .bind(create_model.confidentiality_protection_needs)
            .bind(create_model.integrity_protection_needs)
            .bind(create_model.availability_protection_needs)
            .bind(create_model.description)
            .execute(tx)
            .await?;
        Ok(code)
    }

    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<AssetModel>> {
        Ok(sqlx::query_as!(AssetModel, r#" SELECT * FROM asset"#).fetch_all(db).await?)
    }
    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<AssetDetailModel> {
        let asset: AssetModel = sqlx::query_as!(AssetModel, r#"SELECT * FROM asset WHERE code = $1"#, code.clone()).fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Asset {} not found", code)))?;
        let security_measure_list = SecurityMeasureService::list_by_asset_code(&db, code).await?;

        Ok(AssetDetailModel{
                code: asset.code,
                name: asset.name,
                asset_type: asset.asset_type,
                confidentiality_protection_needs: asset.confidentiality_protection_needs,
                integrity_protection_needs: asset.integrity_protection_needs,
                availability_protection_needs: asset.availability_protection_needs,
                description: asset.description,
                security_measure_list,
            })
    }
}
impl AssetService {
    pub async fn list_for_risk_analysis_process(db: &Pool<Postgres>, rap_code: String) -> ApiResult<Vec<AssetModel>> {
        Ok(
            sqlx::query_as!(AssetModel,
                r#"
                SELECT * FROM asset
                 WHERE EXISTS(SELECT * FROM rap_tour_list
                     WHERE asset.code = rap_tour_list.asset_code
                     AND rap_tour_list.rap_code = $1
                     LIMIT 1
                 )
                "#, rap_code)
                .fetch_all(db)
                .await?
        )
    }

    pub async fn assign_security_measure(
        tx: &mut PgConnection,
        asset_code: String,
        sm_code: String,
    ) -> ApiResult<()> {
        sqlx::query!(r#"INSERT INTO asset_sm_list VALUES ($1, $2)"#, asset_code, sm_code).execute(&mut *tx).await?;

        Ok(())
    }
}
 */