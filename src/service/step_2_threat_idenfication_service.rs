use crate::model::step_2_threat_identification_models::TourModel;
use crate::model::step_2_threat_identification_models::{TourThreatModel, TourThreatReviewModel, TourThreatSummaryModel};
use crate::enums::step_2_threat_identification_enums::ThreatRelevance;
use crate::model::step_2_threat_identification_models::{TourThreatIdentificationModel};
use crate::service::ApiResult;
use sqlx::{PgConnection, Pool, Postgres};
use sqlx::postgres::PgRow;
use crate::model::asset_model::AssetModel;
use crate::model::risk_analysis_process_models::CodeNameModel;
use crate::model::threat_models::ThreatModel;
use crate::service::asset_service::AssetService;

pub struct Step2ThreatIdentificationService;

impl Step2ThreatIdentificationService {
    pub async fn list_threat_identification(
        db: &Pool<Postgres>,
        rap_code: String,
        tour_code: String,
    ) -> ApiResult<Vec<TourThreatModel>> {
        Ok(sqlx::query_as!(TourThreatModel, r#"
            SELECT
                threat.code as threat_code,
                threat.name as threat_name,
                tour_threat_list.relevance,
                tour_threat_list.explanation
            FROM tour_threat_list
            INNER JOIN threat ON threat.code = tour_threat_list.threat_code
            WHERE rap_code = $1 AND tour_code = $2
            "#, rap_code.clone(), tour_code.clone())
            .fetch_all(db)
            .await?
        )
    }

    pub async fn threat_list(
        db: &Pool<Postgres>,
        rap_code: String,
    ) -> ApiResult<Vec<CodeNameModel>> {
        Ok(
            sqlx::query_as!(CodeNameModel, r#"
            SELECT DISTINCT ON(code)
                threat.code,
                threat.name
            FROM tour_threat_list
            INNER JOIN threat ON threat.code = tour_threat_list.threat_code
            WHERE rap_code = $1"#, rap_code.clone())
                .fetch_all(db)
                .await?
        )
    }

    pub async fn tour_list(
        db: &Pool<Postgres>,
        rap_code: String,
    ) -> ApiResult<Vec<CodeNameModel>> {
        Ok(
        sqlx::query_as!(CodeNameModel, r#"
            SELECT DISTINCT ON(asset.code)
                asset.code,
                asset.name
            FROM tour_threat_list
            INNER JOIN threat ON threat.code = tour_threat_list.threat_code
            INNER JOIN asset on asset.code = tour_threat_list.tour_code
            WHERE rap_code = $1
        "#, rap_code)
            .fetch_all(db)
            .await?
        )
    }

    pub async fn list_tour_threat_identification(
        db: &Pool<Postgres>,
        rap_code: String,
    ) -> ApiResult<TourThreatSummaryModel> {
        let mut tour_threat_list: Vec<TourThreatIdentificationModel> = Vec::new();
        for tour in Self::tour_list(&db, rap_code.clone()).await? {
            tour_threat_list.push( TourThreatIdentificationModel{
                tour_code: tour.code.clone(),
                tour_name: tour.name.clone(),
                threat_list: Self::list_threat_identification(&db, rap_code.clone(), tour.code.clone()).await?,
            })
        }

        Ok(TourThreatSummaryModel{
            threat_list: Self::threat_list(&db, rap_code).await?,
            tour_threat_list,
        })
    }
    pub async fn threat_review(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        threat_code: String,
        review: TourThreatReviewModel,
    ) -> ApiResult<()> {
        sqlx::query(
            r#"DELETE FROM tour_threat_list WHERE rap_code = $1 AND tour_code = $2 AND threat_code = $3"#,
        )
            .bind(rap_code.clone())
            .bind(tour_code.clone())
            .bind(threat_code.clone())
            .execute(&mut *tx)
            .await?;

        if (review.relevance == ThreatRelevance::Irrelevant) {
            return Ok(());
        }

        sqlx::query(r#" INSERT INTO tour_threat_list VALUES ($1,$2,$3,$4,$5)"#)
            .bind(rap_code.clone())
            .bind(tour_code.clone())
            .bind(threat_code.clone())
            .bind(review.relevance)
            .bind(review.explanation)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }
}
