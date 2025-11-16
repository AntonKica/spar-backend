use std::fmt::format;
use sqlx::PgConnection;
use spar_backend::configuration::AppConfig;
use spar_backend::create_connection;
use spar_backend::enums::asset_enums::AssetType;
use spar_backend::enums::fulfilled_threat_enums::TimeCostUnit;
use spar_backend::enums::ProtectionNeeds;
use spar_backend::model::asset_model::AssetCreateModel;
use spar_backend::model::fulfilled_threat_models::FulfilledThreatCreateModel;
use spar_backend::model::security_measure_models::SecurityMeasureCreateModel;
use spar_backend::service::asset_service::AssetService;
use spar_backend::service::fulfilled_threat_service::FulfilledThreatService;
use spar_backend::service::GeneralService;
use spar_backend::service::risk_analysis_process_service::RiskAnalysisProcessService;
use spar_backend::service::security_measure_service::SecurityMeasureService;

async fn clear_database(tx: &mut PgConnection) {
    sqlx::query(r#"DELETE FROM risk_analysis_process_tour_list"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_analysis_process"#).execute(&mut *tx).await.unwrap();

    sqlx::query(r#"DELETE FROM asset_fulfilled_threat_list"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM asset_security_measure_list"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM fulfilled_threat"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM security_measure"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM asset"#).execute(&mut *tx).await.unwrap();
}
#[tokio::test]
async fn create_assets() {
    let config = AppConfig::from_env();
    let db = create_connection(&config).await;
    let mut tx = db.begin().await.unwrap();

    clear_database(&mut *tx).await;

    let bp = AssetService::create(&mut *tx, AssetCreateModel{
        name: "App developement process".to_string(),
        asset_type: AssetType::BusinessProcess,
        confidentiality_protection_needs: ProtectionNeeds::Normal,
        integrity_protection_needs: ProtectionNeeds::High,
        availability_protection_needs: ProtectionNeeds::VeryHigh,
        description: "Zakladný proces pre našu organizáciu".to_string(),
    }).await.unwrap();
    AssetService::create(&mut *tx, AssetCreateModel{
        name: "Dumb switch".to_string(),
        asset_type: AssetType::ItSystem,
        confidentiality_protection_needs: ProtectionNeeds::High,
        integrity_protection_needs: ProtectionNeeds::Normal,
        availability_protection_needs: ProtectionNeeds::VeryHigh,
        description: "Switch pre naše PC".to_string(),
    }).await.unwrap();
    AssetService::create(&mut *tx, AssetCreateModel{
        name: "Internal internet links".to_string(),
        asset_type: AssetType::CommunicationsChannel,
        confidentiality_protection_needs: ProtectionNeeds::Normal,
        integrity_protection_needs: ProtectionNeeds::Normal,
        availability_protection_needs: ProtectionNeeds::VeryHigh,
        description: "Ethernety pre naše PC a servery".to_string(),
    }).await.unwrap();
    AssetService::create(&mut *tx, AssetCreateModel{
        name: "Server room".to_string(),
        asset_type: AssetType::Room,
        confidentiality_protection_needs: ProtectionNeeds::High,
        integrity_protection_needs: ProtectionNeeds::High,
        availability_protection_needs: ProtectionNeeds::VeryHigh,
        description: "servery, kde bežia vecičky".to_string(),
    }).await.unwrap();
    AssetService::create(&mut *tx, AssetCreateModel{
        name: "Office Suite".to_string(),
        asset_type: AssetType::ItApplication,
        confidentiality_protection_needs: ProtectionNeeds::Normal,
        integrity_protection_needs: ProtectionNeeds::Normal,
        availability_protection_needs: ProtectionNeeds::High,
        description: "kancelárske nástroje".to_string(),
    }).await.unwrap();

    let fth = FulfilledThreatService::create(&mut *tx, FulfilledThreatCreateModel {
                                       elementary_threat_code: Some("G-17".to_owned()),
                                       specific_threat_code: None,
                                       time_cost: Some(1),
                                       time_cost_unit: Some(TimeCostUnit::Weeks),
                                       monetary_cost: Some(2000),
                                       description: "horelo".to_owned(),
    }).await.unwrap();

    let sm = SecurityMeasureService::create(&mut tx, SecurityMeasureCreateModel{
        name: "prvé bezpečnostné opatrenie".to_owned(),
        description: "rutinné bezpečnostné opatrenie".to_owned(),
        confidentiality_protected: true,
        integrity_protected: false,
        availability_protected: true,
    }).await.unwrap();

    AssetService::assign_fulfilled_threat(&mut *tx, bp.clone(), fth).await.unwrap();
    AssetService::assign_security_measure(&mut *tx, bp.clone(), sm).await.unwrap();

    let rap = RiskAnalysisProcessService::create(&mut *tx).await.unwrap();
    RiskAnalysisProcessService::set_tour(&mut *tx, rap.clone(), vec![bp.clone()]).await.unwrap();

    tx.commit().await.unwrap();
}