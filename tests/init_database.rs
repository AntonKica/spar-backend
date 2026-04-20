use sqlx::PgConnection;
use spar_backend::configuration::AppConfig;
use spar_backend::create_connection;
use spar_backend::enums::{Impact, Likelihood, RiskAnalysisState};
use spar_backend::model::asset_model::AssetModelCreate;
use spar_backend::service::asset_service::AssetService;
use spar_backend::service::{ApiResult, GeneralService};
use spar_backend::service::risk_analysis_service::{RiskAnalysisService, RiskClassificationUpdate};

async fn clear_database(tx: &mut PgConnection) {
    sqlx::query(r#"DELETE FROM risk_analysis_threat"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_analysis_asset"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_analysis"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM asset"#).execute(&mut *tx).await.unwrap();
}

async fn create_assets(tx: &mut PgConnection) {
    let workplace_laptop = AssetService::create(&mut *tx, AssetModelCreate {
        name: "Workplace laptop".to_string(),
        description: "General laptop for employees enabling remote work".to_string(),
        module: "SYS-3-1".to_string(),
    }).await.unwrap();
    let admin_laptop = AssetService::create(&mut *tx, AssetModelCreate {
        name: "Administrator Laptop".to_string(),
        description: "Administrator laptop for server admins for remote server management".to_string(),
        module: "SYS-3-1".to_string(),
    }).await.unwrap();

    let employees = AssetService::create(&mut *tx, AssetModelCreate {
        name: "All the employees".to_string(),
        description: "All the employees of our organization".to_string(),
        module: "ORP-2".to_string(),
    }).await.unwrap();
}

#[tokio::test]
async fn create_scenarios() {
    let config = AppConfig::from_env();
    let db = create_connection(&config).await;
    let mut tx = db.begin().await.unwrap();

    clear_database(&mut *tx).await;
    create_assets(&mut *tx).await;

    create_scenario_threat_identification(&mut *tx).await;
    create_scenario_risk_classification(&mut *tx).await;

    tx.commit().await.unwrap();
}

async fn create_scenario_threat_identification(tx: &mut PgConnection) -> String {
    let ra =  RiskAnalysisService::create(&mut *tx).await.unwrap();
    RiskAnalysisService::sync_threats(&mut *tx, ra.clone(), "SYS-3-1".to_string(), vec!["G-04".to_string(), "G-14".to_string(), "G-25".to_string(), "G-26".to_string()]).await.unwrap();
    RiskAnalysisService::sync_threats(&mut *tx, ra.clone(), "ORP-2".to_string(), vec!["G-04".to_string(), "G-14".to_string(), "G-33".to_string()]).await.unwrap();

    ra
}

async fn create_scenario_risk_classification(tx: &mut PgConnection) {
    let ra = create_scenario_threat_identification(&mut *tx).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::ThreatIdentification).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-04".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Limited, evaluation: "".to_string() }).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-25".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Significant, evaluation: "".to_string() }).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-26".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Limited, evaluation: "".to_string() }).await.unwrap();

    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-14".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Limited, evaluation: "".to_string() }).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-33".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::LifeThreatening, evaluation: "".to_string() }).await.unwrap();
}
