use crate::model::step_2_threat_identification_models::StSummaryModel;
use crate::enums::step_2_threat_identification_enums::ElementaryThreatRelevance;
use crate::model::step_2_threat_identification_models::{TourEtModel, TourEtReviewModel, TourStReviewModel, TourStModel, TourThreatIdentificationModel, ThreatSummaryModel, EtSummaryModel};
use crate::service::ApiResult;
use sqlx::{PgConnection, Pool, Postgres};
use crate::model::asset_model::AssetModel;
use crate::service::asset_service::AssetService;

pub struct Step2ThreatIdentificationService;

impl Step2ThreatIdentificationService {
    pub async fn list_threat_identification(
        db: &Pool<Postgres>,
        rap_code: String,
    ) -> ApiResult<Vec<TourThreatIdentificationModel>> {
        let tour_list: Vec<AssetModel> = AssetService::list_for_risk_analysis_process(&db, rap_code.clone()).await?;

        let mut res: Vec<TourThreatIdentificationModel> = Vec::new();
        for tour in tour_list {
            let et_list = sqlx::query_as!(TourEtModel, r#"SELECT * FROM tour_et_list WHERE rap_code = $1 AND tour_code = $2 "#, rap_code.clone(), tour.code.clone())
                .fetch_all(db)
                .await?;
            let st_list = sqlx::query_as!(TourStModel, r#"
        SELECT
            tour_st_list.tour_code,
            tour_st_list.st_code,
            specific_threat.name as st_name,
            tour_st_list.explanation
        FROM tour_st_list
        INNER JOIN specific_threat ON specific_threat.code = tour_st_list.st_code
        WHERE rap_code = $1 AND tour_code = $2
        "#, rap_code.clone(), tour.code.clone())
                .fetch_all(db)
                .await?;

            res.push(TourThreatIdentificationModel{
                tour_code: tour.code,
                tour_name: tour.name,
                et_list,
                st_list,
            })
        }

        Ok(res)
    }

    pub async fn elementary_threat_review(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        et_code: String,
        review: TourEtReviewModel,
    ) -> ApiResult<()> {
        sqlx::query(
            r#"DELETE FROM tour_et_list WHERE rap_code = $1 AND tour_code = $2 AND et_code = $3"#,
        )
            .bind(rap_code.clone())
            .bind(tour_code.clone())
            .bind(et_code.clone())
            .execute(&mut *tx)
            .await?;

        if (review.relevance == ElementaryThreatRelevance::Irrelevant) {
            return Ok(());
        }

        sqlx::query(r#" INSERT INTO tour_et_list VALUES ($1,$2,$3,$4,$5)"#)
            .bind(rap_code.clone())
            .bind(tour_code.clone())
            .bind(et_code.clone())
            .bind(review.relevance)
            .bind(review.explanation)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }

    pub async fn specific_threat_review(
        tx: &mut PgConnection,
        rap_code: String,
        tour_code: String,
        st_code: String,
        review: TourStReviewModel,
    ) -> ApiResult<()> {
        sqlx::query(
            r#"DELETE FROM tour_st_list WHERE rap_code = $1 AND tour_code = $2 AND st_code = $3"#,
        )
            .bind(rap_code.clone())
            .bind(tour_code.clone())
            .bind(st_code.clone())
            .execute(&mut *tx)
            .await?;

        if (!review.relevant) {
            return Ok(());
        }

        sqlx::query(r#" INSERT INTO tour_st_list VALUES ($1,$2,$3,$4)"#)
            .bind(rap_code.clone())
            .bind(tour_code.clone())
            .bind(st_code.clone())
            .bind(review.explanation)
            .execute(&mut *tx)
            .await?;

        Ok(())
    }

    pub async fn summary(
        db: &Pool<Postgres>,
        rap_code: String,
    ) -> ApiResult<ThreatSummaryModel> {
        let tour_list: Vec<AssetModel> = AssetService::list_for_risk_analysis_process(&db, rap_code.clone()).await?;

        let et_list = sqlx::query_as!(EtSummaryModel, r#"SELECT DISTINCT ON(et_code) et_code, relevance FROM tour_et_list WHERE rap_code = $1"#, rap_code.clone()).fetch_all(db) .await?;
        let st_list = sqlx::query_as!(StSummaryModel, r#"SELECT DISTINCT ON(st_code) st_code, specific_threat.name as st_name FROM tour_st_list INNER JOIN specific_threat ON tour_st_list.st_code = specific_threat.code WHERE rap_code = $1"#, rap_code.clone()).fetch_all(db) .await?;

        Ok(ThreatSummaryModel{
            et_list,
            st_list,
        })
    }
}
