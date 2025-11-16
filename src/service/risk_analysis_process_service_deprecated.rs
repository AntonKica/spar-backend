use crate::model::{RiskAnalysisProcessCreateModel, TOURElementaryThreatUpdateModel, TOURSpecificThreatCreateModel, TOURSpecificThreatModel, TOURSpecificThreatOverviewModel, TargetObjectUnderReviewCreateModel};
use crate::response::RiskAnalysisProcessResponse;
use crate::service::{next_code_for, next_code_for_db, ApiError, ApiResult, GeneralService};
use crate::workflow::create_risk_analysis_process_workflow;
use crate::workflow_model::{create_workflow_model, WorkflowModel};
use chrono::{NaiveDate, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{Executor, PgConnection, Pool, Postgres};
use crate::enums::ElementaryThreatRelevance;
use crate::service::risk_classification_service::RiskClassificationService;
use crate::service::risk_treatment_service::RiskTreatmentService;

pub struct RiskAnalysisProcessService;

#[derive(Clone)]
struct RiskAnalysisProcessRow {
    pub code: String,
    pub created_on: NaiveDate,
    pub workflow: Value,
}

pub struct RiskAnalysisProcessModel {
    pub code: String,
    pub created_on: NaiveDate,
    pub workflow: WorkflowModel,
    pub target_objects_under_review: Vec<String>
}

impl RiskAnalysisProcessModel {
    fn from_row(row: RiskAnalysisProcessRow, target_objects_under_review: Vec<TargetObjectUnderReviewCreateModel>) -> RiskAnalysisProcessModel {
        RiskAnalysisProcessModel {
            code: row.code.clone(),
            created_on: row.created_on.clone(),
            // TODO please dont fail
            workflow: serde_json::from_value(row.workflow.clone()).unwrap(),
            target_objects_under_review: target_objects_under_review.into_iter().map(|t| t.asset_code).collect(),
        }
    }
}

impl GeneralService<RiskAnalysisProcessResponse, RiskAnalysisProcessCreateModel> for RiskAnalysisProcessService {
    const TABLE_NAME: &'static str = "risk_analysis_process";
    const CODE_PREFIX: &'static str = "RAP";
    const CODE_DIGITS: usize = 8;
    async fn create(
        tx: &mut PgConnection,
        create_model: RiskAnalysisProcessCreateModel
    ) -> ApiResult<String> {
        let code = match next_code_for(Self::TABLE_NAME, Self::CODE_PREFIX, Self::CODE_DIGITS, &mut *tx).await {
            Ok(val) => val,
            Err(_) => return Err(ApiError::Internal)
        };

        let created_on = Utc::now().date_naive();
        let workflow = json!(create_workflow_model(&create_risk_analysis_process_workflow()));

        sqlx::query!(
        r#"INSERT INTO risk_analysis_process(code, created_on, workflow) VALUES ($1,$2,$3)"#,
            code,
            created_on,
            workflow
        )
            .execute(&mut *tx)
            .await?;


        sqlx::query(r#"
            INSERT INTO target_object_under_review (
                risk_analysis_process_code,
                asset_code
            ) SELECT * FROM UNNEST(
                $1::CHAR(8)[],
                $2::VARCHAR(20)[]
            );"#)
            .bind(vec![code.to_owned(); create_model.target_objects_under_review.len()])
            .bind(&create_model.target_objects_under_review)
            .fetch_all(&mut *tx)
            .await?;

        sqlx::query!(
            r#"
INSERT INTO tour_elementary_threat
SELECT target_object_under_review.risk_analysis_process_code AS risk_analysis_process_code,
       target_object_under_review.asset_code as asset_code,
       it_grundschutz_elementary_threat.code as it_grundschutz_elementary_threat_code,
       $2,
       '',
       FALSE
FROM it_grundschutz_elementary_threat
INNER JOIN target_object_under_review ON target_object_under_review.risk_analysis_process_code = $1
"#,
            code.clone(),
            ElementaryThreatRelevance::IRRELEVANT as i32
        )
            .execute(&mut *tx)
            .await?;

        sqlx::query(r#"
            INSERT INTO tour_specific_threat_overview (
                risk_analysis_process_code,
                asset_code,
                reviewed
            ) SELECT * FROM UNNEST(
                $1::CHAR(8)[],
                $2::VARCHAR(20)[],
                $3::BOOLEAN[]
            );
            "#
        )
            .bind(vec![code.to_owned(); create_model.target_objects_under_review.len()])
            .bind(&create_model.target_objects_under_review)
            .bind(vec![false; create_model.target_objects_under_review.len()])
            .fetch_all(&mut *tx)
            .await?;

        Ok(code)
    }
    async fn list(db: &Pool<Postgres>) -> ApiResult<Vec<RiskAnalysisProcessResponse>>
    {
        let rows = sqlx::query_as!(RiskAnalysisProcessRow, r#"SELECT * FROM risk_analysis_process"#)
            .fetch_all(db)
            .await?;

        let mut list: Vec<RiskAnalysisProcessResponse> = vec![];
        for row in rows {
            let target_objects_under_review = sqlx::query_as!(TargetObjectUnderReviewCreateModel, r#"SELECT * FROM target_object_under_review WHERE risk_analysis_process_code = $1"#, row.code)
                .fetch_all(db)
                .await?;

            list.push(RiskAnalysisProcessResponse::from(RiskAnalysisProcessModel::from_row(row.clone(), target_objects_under_review)));
        };

        Ok(list)
    }

    async fn get_by_code(db: &Pool<Postgres>, code: String) -> ApiResult<RiskAnalysisProcessResponse> {
        let row = sqlx::query_as!(RiskAnalysisProcessRow, r#"SELECT * FROM risk_analysis_process WHERE code = $1"#, code)
            .fetch_optional(db)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("RiskAnalysisProcess with code {} not found", code)))?;
        let target_objects_under_review = sqlx::query_as!(TargetObjectUnderReviewCreateModel, r#"SELECT * FROM target_object_under_review WHERE risk_analysis_process_code = $1"#, row.code)
            .fetch_all(db)
            .await?;
        Ok(RiskAnalysisProcessResponse::from(RiskAnalysisProcessModel::from_row(row, target_objects_under_review)))
    }
}

pub struct TOURThreatOverviewModel {
    pub asset_code: String,
    pub asset_name: String,
    pub confidentiality_protection_needs: i32,
    pub integrity_protection_needs: i32,
    pub availability_protection_needs: i32,
    // TODO BUG WITH QUERY WHICH SHOULD EXPECT BOOL instead of OPTION
    pub identified_basic_threat: Option<bool>,
    pub identified_specific_threat: Option<bool>,
}

#[derive(Serialize)]
pub struct TOURThreatOverviewResponse {
    pub asset_code: String,
    pub asset_name: String,
    pub confidentiality_protection_needs: i32,
    pub integrity_protection_needs: i32,
    pub availability_protection_needs: i32,
    // TODO BUG WITH QUERY WHICH SHOULD EXPECT BOOL instead of OPTION
    // https://github.com/launchbadge/sqlx/issues/4065
    pub identified_basic_threat: bool,
    pub identified_specific_threat: bool,
}
impl From<TOURThreatOverviewModel> for TOURThreatOverviewResponse {
    fn from(model: TOURThreatOverviewModel) -> Self {
        Self {
            asset_name: model.asset_name,
            asset_code: model.asset_code,
            confidentiality_protection_needs: model.confidentiality_protection_needs,
            integrity_protection_needs: model.integrity_protection_needs,
            availability_protection_needs: model.availability_protection_needs,
            identified_basic_threat: model.identified_basic_threat.unwrap(),
            identified_specific_threat: model.identified_specific_threat.unwrap(),
        }
    }
}

pub struct TOURElementaryThreatModel {
    pub elementary_threat_code: String,
    pub relevance: i32,
    pub comment: String,
    pub reviewed: bool,
}

#[derive(Serialize)]
pub struct TOURElementaryThreatResponse {
    pub elementary_threat_code: String,
    pub relevance: i32,
    pub comment: String,
    pub reviewed: bool,
}

impl From<TOURElementaryThreatModel> for TOURElementaryThreatResponse {
    fn from(model: TOURElementaryThreatModel) -> Self {
        Self {
            elementary_threat_code: model.elementary_threat_code,
            relevance: model.relevance,
            comment: model.comment,
            reviewed: model.reviewed,
        }
    }
}

#[derive(Serialize)]
pub struct TOURSpecificThreatResponse {
    pub code: String,
    pub name: String,
    pub description: String,
}

impl From<TOURSpecificThreatModel> for TOURSpecificThreatResponse {
    fn from(model: TOURSpecificThreatModel) -> Self {
        Self {
            code: model.code,
            name: model.name,
            description: model.description,
        }
    }
}

#[derive(Serialize)]
pub struct TOURSpecificThreatOverviewResponse {
    pub reviewed: bool,
}

impl From<TOURSpecificThreatOverviewModel> for TOURSpecificThreatOverviewResponse {
    fn from(model: TOURSpecificThreatOverviewModel) -> Self {
        Self {
            reviewed: model.reviewed,
        }
    }
}

impl RiskAnalysisProcessService {
    pub async fn get_threat_overview(db: &Pool<Postgres>, code: String) -> ApiResult<Vec<TOURThreatOverviewResponse>>
    {
        let res: Vec<TOURThreatOverviewModel> = sqlx::query_as!(TOURThreatOverviewModel,
            r#"
SELECT
    asset.code AS asset_code,
    asset.name AS asset_name,
    asset.confidentiality_protection_needs,
    asset.integrity_protection_needs,
    asset.availability_protection_needs,
    CASE
        WHEN EXISTS (
            SELECT 1
            FROM tour_elementary_threat
            WHERE tour_elementary_threat.risk_analysis_process_code = $1
              AND tour_elementary_threat.asset_code = asset.code
              AND reviewed = FALSE
        ) THEN FALSE
        ELSE TRUE
        END AS identified_basic_threat,
    tour_specific_threat_overview.reviewed AS identified_specific_threat
FROM asset
         INNER JOIN target_object_under_review ON asset.code = target_object_under_review.asset_code
         INNER JOIN tour_specific_threat_overview ON asset.code = tour_specific_threat_overview.asset_code AND tour_specific_threat_overview.risk_analysis_process_code = $1
WHERE target_object_under_review.risk_analysis_process_code = $1
ORDER BY CODE
            "#,
            code
        ).fetch_all(db).await?;

        Ok(res.into_iter().map(TOURThreatOverviewResponse::from).collect())
    }


    pub async fn get_elementary_threat_list(db: &Pool<Postgres>, code: String, asset: String) -> ApiResult<Vec<TOURElementaryThreatResponse>>
    {
        let res: Vec<TOURElementaryThreatModel> =  sqlx::query_as!(TOURElementaryThreatModel,
            r#"
            SELECT
            iget.code AS elementary_threat_code,
            tet.relevance,
            tet.comment,
            tet.reviewed
            FROM tour_elementary_threat AS tet
            INNER JOIN it_grundschutz_elementary_threat AS iget ON iget.code = tet.it_grundschutz_elementary_threat_code
            INNER JOIN asset AS a ON a.code = tet.asset_code
            WHERE tet.risk_analysis_process_code = $1 AND tet.asset_code = $2
            ORDER BY iget.code
            "#,
            code,
            asset
        ).fetch_all(db).await?;

        Ok(res.into_iter().map(TOURElementaryThreatResponse::from).collect())
    }

    pub async fn update_elementary_threat_list(db: &Pool<Postgres>, code: String, asset: String, update: Vec<TOURElementaryThreatUpdateModel>) -> ApiResult<()>
    {
        sqlx::query(r#"
            INSERT INTO tour_elementary_threat (
                risk_analysis_process_code,
                asset_code,
                it_grundschutz_elementary_threat_code,
                relevance,
                comment,
                reviewed
            ) SELECT * FROM UNNEST(
                $1::CHAR(8)[],
                $2::VARCHAR(20)[],
                $3::VARCHAR(20)[],
                $4::INTEGER[],
                $5::TEXT[],
                $6::BOOLEAN[]
            ) ON CONFLICT (risk_analysis_process_code, asset_code, it_grundschutz_elementary_threat_code) DO UPDATE SET
                relevance = EXCLUDED.relevance,
                comment = EXCLUDED.comment,
                reviewed = EXCLUDED.reviewed
            "#
        )
            .bind(vec![code.to_owned(); update.len()])
            .bind(vec![asset.to_owned(); update.len()])
            .bind(update.iter().map(|i| i.elementary_threat_code.clone()).collect::<Vec<String>>())
            .bind(update.iter().map(|i| i.relevance.clone()).collect::<Vec<i32>>())
            .bind(update.iter().map(|i| i.comment.clone()).collect::<Vec<String>>())
            .bind(update.iter().map(|i| i.reviewed.clone()).collect::<Vec<bool>>())
            .fetch_all(db)
            .await?;

        Ok(())
    }

    pub async fn get_specific_threat_list(db: &Pool<Postgres>, code: String, asset: String) -> ApiResult<Vec<TOURSpecificThreatResponse>>
    {
        let res: Vec<TOURSpecificThreatModel> =  sqlx::query_as!(TOURSpecificThreatModel,
            r#"
            SELECT code, name, description, confidentiality_impaired,integrity_impaired, availability_impaired
            FROM tour_specific_threat
            WHERE risk_analysis_process_code = $1 AND asset_code = $2
            ORDER BY code
            "#,
            code,
            asset
        ).fetch_all(db).await?;

        Ok(res.into_iter().map(TOURSpecificThreatResponse::from).collect())
    }
    pub async fn create_specific_threat(db: &Pool<Postgres>, rap: String, asset: String, create: TOURSpecificThreatCreateModel) -> ApiResult<String>
    {
        let code = next_code_for_db("tour_specific_threat","THR", 10, &db).await?;

        sqlx::query!(
        r#"INSERT INTO tour_specific_threat
        (
        code,
        risk_analysis_process_code,
        asset_code,
        name,
        description,
        confidentiality_impaired,
        integrity_impaired,
        availability_impaired
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)"#,
            code,
            rap,
            asset,
            create.name,
            create.description,
            create.confidentiality_impaired,
            create.integrity_impaired,
            create.availability_impaired
        )
            .execute(db)
            .await?;

        Ok(code)
    }
    pub async fn delete_specific_threat(db: &Pool<Postgres>, rap: String, asset: String, threat: String) -> ApiResult<()>
    {
        sqlx::query!(
            r#"
            DELETE FROM tour_specific_threat
            WHERE code = $1 AND risk_analysis_process_code = $2 AND asset_code = $3
            "#,
            threat,
            rap,
            asset
        ).fetch_all(db).await?;

        Ok(())
    }
    pub async fn update_specific_threat(db: &Pool<Postgres>, rap: String, asset: String, threat: String, update: TOURSpecificThreatCreateModel) -> ApiResult<()>
    {
        sqlx::query!(
        r#"UPDATE tour_specific_threat
        SET name = $4, description = $5
        WHERE code = $1 AND risk_analysis_process_code = $2 AND asset_code = $3"#,
            threat,
            rap,
            asset,
            update.name,
            update.description
        )
            .execute(db)
            .await?;

        Ok(())
    }
    pub async fn get_specific_threat_overview(db: &Pool<Postgres>, rap: String, asset: String) -> ApiResult<(TOURSpecificThreatOverviewResponse)>
    {
        let res: TOURSpecificThreatOverviewModel =  sqlx::query_as!(TOURSpecificThreatOverviewModel,
            r#"
            SELECT reviewed
            FROM tour_specific_threat_overview
            WHERE risk_analysis_process_code = $1 AND asset_code = $2
            "#,
            rap,
            asset
        )
            .fetch_optional(db).await?
            .ok_or_else(|| ApiError::NotFound(format!("Specific threat with code {} asset {} not found", rap, asset)))?;

        Ok(TOURSpecificThreatOverviewResponse::from(res))
    }

    pub async fn specific_threat_overview_set_reviewed(db: &Pool<Postgres>, rap: String, asset: String, value: bool) -> ApiResult<()>
    {
        sqlx::query!(
            r#"
            UPDATE tour_specific_threat_overview
            SET reviewed = $3
            WHERE risk_analysis_process_code = $1 AND asset_code = $2
            "#,
            rap,
            asset,
            value

        )
            .execute(db).await?;

        Ok(())
    }

    pub async fn step_1_threat_overview_finish (
        tx: &mut PgConnection,
        code: String) -> ApiResult<()> {
        RiskClassificationService::create_risk_classifications(&mut *tx, code).await?;
        Ok(())

    }

    pub async fn step_2_risk_classification_finish (
        tx: &mut PgConnection,
        code: String) -> ApiResult<()> {
        RiskTreatmentService::create_risk_treatments(&mut *tx, code).await?;
        Ok(())
    }
}