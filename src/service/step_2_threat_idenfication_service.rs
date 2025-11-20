use crate::enums::step_2_threat_identification_enums::ElementaryThreatRelevance;
use crate::model::step_2_threat_identification_models::{TourEtModel, TourEtReviewModel, TourThreatIdentification};
use crate::service::ApiResult;
use sqlx::{PgConnection, Pool, Postgres};

pub struct Step2ThreatIdentificationService;

impl Step2ThreatIdentificationService {
    pub async fn list_threat_identification(
        db: &Pool<Postgres>,
        rap_code: String,
    ) -> ApiResult<TourThreatIdentification> {
        let et_list  = sqlx::query_as!(TourEtModel, r#" SELECT * FROM tour_et_list WHERE rap_code = $1 "#, rap_code)
            .fetch_all(db)
            .await?;

        Ok(TourThreatIdentification {
            et_list
        })
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
}
