use sqlx::PgConnection;
use spar_backend::configuration::AppConfig;
use spar_backend::create_connection;
use spar_backend::enums::{Impact, ImplementationStatus, Likelihood, ProtectionRequirement, RiskAnalysisState, RiskTreatmentType};
use spar_backend::model::asset_model::AssetCreateModel;
use spar_backend::service::asset_service::AssetService;
use spar_backend::service::{ApiResult, GeneralService};
use spar_backend::service::it_grundschutz_check_service::ItGrundschutCheckService;
use spar_backend::service::risk_analysis_service::{RiskAnalysisService, RiskAssessmentUpdateModel, RiskClassificationUpdate};
use spar_backend::service::security_measure_service::{SecurityMeasureCreate, SecurityMeasureService};

async fn clear_database(tx: &mut PgConnection) {
    sqlx::query(r#"TRUNCATE TABLE risk_treatment CASCADE"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"TRUNCATE TABLE security_measure CASCADE "#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"TRUNCATE TABLE risk_analysis_threat CASCADE "#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"TRUNCATE TABLE risk_analysis_asset CASCADE "#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"TRUNCATE TABLE risk_analysis CASCADE "#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"TRUNCATE TABLE asset CASCADE"#).execute(&mut *tx).await.unwrap();

    sqlx::query(r#"ALTER SEQUENCE asset_code_seq RESTART"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"ALTER SEQUENCE risk_analysis_code_seq RESTART"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"ALTER SEQUENCE security_measure_avd_seq RESTART"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"ALTER SEQUENCE security_measure_red_seq RESTART"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"ALTER SEQUENCE security_measure_tsf_seq RESTART"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"ALTER SEQUENCE security_measure_acp_seq RESTART"#).execute(&mut *tx).await.unwrap();
}

async fn create_assets(tx: &mut PgConnection) {
    let workplace_laptop = AssetService::create(&mut *tx, AssetCreateModel {
        name: "Všeobecný notebook".to_string(),
        description: "Notebook pre zamestnancov na umožnenie práce z domu".to_string(),
        module: "SYS-3-1".to_string(),
        confidentiality_protection_requirement: ProtectionRequirement::High,
        integrity_protection_requirement: ProtectionRequirement::Low,
        availability_protection_requirement: ProtectionRequirement::High,
        confidentiality_protection_requirement_description: "Umožňuje prístup k dôverným informáciám a systémom".to_string(),
        integrity_protection_requirement_description: "".to_string(),
        availability_protection_requirement_description: "Vysoká dostupnosť je integrálna na včasné riešenie problémov.".to_string(),
    }).await.unwrap();
    let admin_laptop = AssetService::create(&mut *tx, AssetCreateModel {
        name: "Administrátorský notebook".to_string(),
        description: "Notebook pre serverových administrátorov umožňujúci vzdialenú správu serverov pri riešení problémov".to_string(),
        module: "SYS-3-1".to_string(),
        confidentiality_protection_requirement: ProtectionRequirement::VeryHigh,
        integrity_protection_requirement: ProtectionRequirement::Low,
        availability_protection_requirement: ProtectionRequirement::VeryHigh,
        confidentiality_protection_requirement_description: "Umožňuje privilegovaný administrátorský prístup do systémov s dôvernými informáciami.".to_string(),
        integrity_protection_requirement_description: "".to_string(),
        availability_protection_requirement_description: "Vysoká dostupnosť je integrálna na včasné riešenie problémov.".to_string(),
    }).await.unwrap();

    let employees = AssetService::create(&mut *tx, AssetCreateModel {
        name: "Všetci zamesntanci".to_string(),
        description: "Všetci zamesntanci našej organizácie".to_string(),
        module: "ORP-2".to_string(),
        confidentiality_protection_requirement: ProtectionRequirement::High,
        integrity_protection_requirement: ProtectionRequirement::Low,
        availability_protection_requirement: ProtectionRequirement::High,
        confidentiality_protection_requirement_description: "Zamestnanci majú znalosť vysokohodnotného know-how.".to_string(),
        integrity_protection_requirement_description: "".to_string(),
        availability_protection_requirement_description: "Vysoká dostupnosť zamestnancov je potrebná pre riešenie problémov.".to_string(),
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
    create_scenario_risk_treatment(&mut *tx).await;
    create_scenario_it_grundschutz_check(&mut *tx).await;
    create_scenario_completed(&mut *tx).await;

    tx.commit().await.unwrap();
}

async fn create_scenario_threat_identification(tx: &mut PgConnection) -> String {
    let ra =  RiskAnalysisService::create(&mut *tx).await.unwrap();
    RiskAnalysisService::sync_threats(&mut *tx, ra.clone(), "SYS-3-1".to_string(), vec!["G-04".to_string(), "G-14".to_string(), "G-25".to_string(), "G-26".to_string()]).await.unwrap();
    RiskAnalysisService::sync_threats(&mut *tx, ra.clone(), "ORP-2".to_string(), vec!["G-04".to_string(), "G-14".to_string(), "G-33".to_string()]).await.unwrap();

    RiskAnalysisService::set_module_threat_identification_done(&mut *tx, ra.clone(), "SYS-3-1".to_string()).await.unwrap();
    //RiskAnalysisService::set_module_threat_identification_done(&mut *tx, ra.clone(), "ORP-2".to_string()).await.unwrap();

    ra
}

async fn create_scenario_risk_classification(tx: &mut PgConnection) -> String {
    let ra = create_scenario_threat_identification(&mut *tx).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::ThreatIdentification).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-04".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Limited, evaluation: "".to_string() }).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-25".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Significant, evaluation: "".to_string() }).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-26".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Limited, evaluation: "".to_string() }).await.unwrap();

    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-14".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::Limited, evaluation: "".to_string() }).await.unwrap();
    RiskAnalysisService::update_risk_classification(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-33".to_string(), RiskClassificationUpdate{likelihood: Likelihood::Often, impact: Impact::LifeThreatening, evaluation: "".to_string() }).await.unwrap();

    ra
}

async fn create_scenario_risk_treatment(tx: &mut PgConnection) -> String {
    let ra = create_scenario_risk_classification(&mut *tx).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::RiskClassification).await.unwrap();

    let avoid = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Avoid,
        description: "Moved to dust-free compartments".to_string(),
    }).await.unwrap();
    RiskAnalysisService::sync_threat_risk_treatment(&mut *tx, ra.clone(), "G-04".to_string(), RiskTreatmentType::Avoid, vec![avoid]).await.unwrap();

    let treatment_encryption = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "The additional capabilities are drawn from subcontractor providing necessary personel.".to_string(),
    }).await.unwrap();
    let treatment_password = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "Proper password policy is enacted providing strong password management.".to_string(),
    }).await.unwrap();
    RiskAnalysisService::sync_module_threat_risk_treatment(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-14".to_string(), RiskTreatmentType::Reduce, vec![treatment_encryption, treatment_password]).await.unwrap();

    RiskAnalysisService::sync_risk_treatment_requirement_for_module(&mut *tx, ra.clone(), "ORP-2".to_string(), vec!["ORP-2-A1".to_string(), "ORP-2-A7".to_string()]).await.unwrap();
    RiskAnalysisService::sync_risk_treatment_requirement_for_module(&mut *tx, ra.clone(), "SYS-3-1".to_string(), vec!["SYS-3-1-A13".to_string(), "SYS-3-1-A6".to_string()]).await.unwrap();
    let treatment_nda = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "Emplyee are now obliged to sign a NDA".to_string(),
    }).await.unwrap();
    RiskAnalysisService::sync_module_threat_risk_treatment(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-14".to_string(), RiskTreatmentType::Reduce, vec![treatment_nda]).await.unwrap();

    let treatment_outsource = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Transfer,
        description: "The additional capabilities are drawn from subcontractor providing necessary personnel.".to_string(),
    }).await.unwrap();
    RiskAnalysisService::sync_module_threat_risk_treatment(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-33".to_string(), RiskTreatmentType::Transfer, vec![treatment_outsource]).await.unwrap();


    let treatment_guidelines = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "The organisation will publish and update necessary security guidelines.".to_string(),
    }).await.unwrap();
    let treatment_training = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "The employees will enroll in yearly security training workshops.".to_string(),
    }).await.unwrap();
    RiskAnalysisService::sync_org_risk_treatment(&mut *tx, ra.clone(), vec![treatment_guidelines, treatment_training]).await.unwrap();

    ra
}


async fn create_scenario_it_grundschutz_check(tx: &mut PgConnection) -> String {
    let ra = create_scenario_risk_treatment(&mut *tx).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::RiskTreatment).await.unwrap();

    let full_assessment = ItGrundschutCheckService::full_assessment(&mut *tx, ra.clone()).await.unwrap();

    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.org.as_ref().unwrap().items[0].id,
                                                                 RiskAssessmentUpdateModel{
                                                                     status: ImplementationStatus::Full,
                                                                     evaluation: "The security guidelines material are now accessible to every employee.".to_string()
                                                                 }).await.unwrap();
    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.org.as_ref().unwrap().items[1].id,
                                                                 RiskAssessmentUpdateModel{
                                                                     status: ImplementationStatus::None,
                                                                     evaluation: "We have failed in search of an expert in security training.".to_string()
                                                                 }).await.unwrap();

    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.threats[0].items[0].id,
                                                                 RiskAssessmentUpdateModel{
                                                                     status: ImplementationStatus::Partial,
                                                                     evaluation: "We have failed in search of an expert in security training.".to_string()
                                                                 }).await.unwrap();

    ItGrundschutCheckService::update_requirement_assessment(&mut *tx, full_assessment.modules[0].items[0].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Full, evaluation: "".to_string() }).await.unwrap();
    ItGrundschutCheckService::update_requirement_assessment(&mut *tx, full_assessment.modules[0].items[1].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Redundant, evaluation: "".to_string() }).await.unwrap();
    ItGrundschutCheckService::update_requirement_assessment(&mut *tx, full_assessment.modules[1].items[0].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Full, evaluation: "".to_string() }).await.unwrap();
    ItGrundschutCheckService::update_requirement_assessment(&mut *tx, full_assessment.modules[1].items[1].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Redundant, evaluation: "".to_string() }).await.unwrap();
    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.module_threats[0].items[0].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Full, evaluation: "".to_string() }).await.unwrap();
    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.module_threats[1].items[0].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Full, evaluation: "".to_string() }).await.unwrap();
    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.module_threats[2].items[0].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Full, evaluation: "".to_string() }).await.unwrap();
    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.module_threats[2].items[1].id, RiskAssessmentUpdateModel{ status: ImplementationStatus::Redundant, evaluation: "".to_string() }).await.unwrap();

    ra
}

async fn create_scenario_completed(tx: &mut PgConnection) -> String {
    let ra = create_scenario_it_grundschutz_check(&mut *tx).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::ItGrundschutzCheck).await.unwrap();

    ra
}
