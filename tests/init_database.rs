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

#[derive(Clone)]
struct TestRiskTreatments {
    pub avoid: String,
    pub treatment_encryption: String,
    pub treatment_password: String,
    pub treatment_nda: String,
    pub treatment_outsource: String,
    pub treatment_guidelines: String,
    pub treatment_training: String,
}
async fn create_treatments(tx: &mut PgConnection) -> TestRiskTreatments {
    let avoid = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Avoid,
        description: "Presunúť do bezprašných priestorov.".to_string(),
    }).await.unwrap();

    let treatment_encryption = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "Zavedenie šifrovania diskov a prenosných médií.".to_string(),
    }).await.unwrap();
    let treatment_password = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "Zriadenie patričná politiky hesiel, ktorá zabezpečí silnú správu hesiel.".to_string(),
    }).await.unwrap();

    let treatment_nda = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "Zamestnanci sú zaviazaní podpisovať NDA".to_string(),
    }).await.unwrap();

    let treatment_outsource = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Transfer,
        description: "Dodatočné kapacity sú dodané dodávateľom poskytujúci dodatočný personál.".to_string(),
    }).await.unwrap();


    let treatment_guidelines = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "Organizácia publikuje a aktualizuje potrebné bezpečnostné návody.".to_string(),
    }).await.unwrap();
    let treatment_training = SecurityMeasureService::create(&mut *tx, SecurityMeasureCreate {
        treatment: RiskTreatmentType::Reduce,
        description: "Zamestnanci budú každoročné trénovaný bezpečnostnými workshopmi.".to_string(),
    }).await.unwrap();

    TestRiskTreatments {
        avoid,
        treatment_encryption,
        treatment_password,
        treatment_nda,
        treatment_outsource,
        treatment_guidelines,
        treatment_training,
    }
}

#[tokio::test]
async fn create_scenarios() {
    let config = AppConfig::from_env();
    let db = create_connection(&config).await;
    let mut tx = db.begin().await.unwrap();

    clear_database(&mut *tx).await;
    create_assets(&mut *tx).await;
    let treatments = create_treatments(&mut *tx).await;

    create_scenario_threat_identification(&mut *tx).await;
    create_scenario_risk_classification(&mut *tx).await;
    create_scenario_risk_treatment(&mut *tx, treatments.clone()).await;
    create_scenario_it_grundschutz_check(&mut *tx, treatments.clone()).await;
    create_scenario_completed(&mut *tx, treatments.clone()).await;

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

async fn create_scenario_risk_treatment(tx: &mut PgConnection, risk_treatments: TestRiskTreatments) -> String {
    let ra = create_scenario_risk_classification(&mut *tx).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::RiskClassification).await.unwrap();

    RiskAnalysisService::sync_threat_risk_treatment(&mut *tx, ra.clone(), "G-04".to_string(), RiskTreatmentType::Avoid, vec![risk_treatments.avoid]).await.unwrap();
    RiskAnalysisService::sync_module_threat_risk_treatment(&mut *tx, ra.clone(), "SYS-3-1".to_string(), "G-14".to_string(), RiskTreatmentType::Reduce, vec![risk_treatments.treatment_encryption, risk_treatments.treatment_password]).await.unwrap();

    RiskAnalysisService::sync_risk_treatment_requirement_for_module(&mut *tx, ra.clone(), "ORP-2".to_string(), vec!["ORP-2-A1".to_string(), "ORP-2-A7".to_string()]).await.unwrap();
    RiskAnalysisService::sync_risk_treatment_requirement_for_module(&mut *tx, ra.clone(), "SYS-3-1".to_string(), vec!["SYS-3-1-A13".to_string(), "SYS-3-1-A6".to_string()]).await.unwrap();
    RiskAnalysisService::sync_module_threat_risk_treatment(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-14".to_string(), RiskTreatmentType::Reduce, vec![risk_treatments.treatment_nda]).await.unwrap();
    RiskAnalysisService::sync_module_threat_risk_treatment(&mut *tx, ra.clone(), "ORP-2".to_string(), "G-33".to_string(), RiskTreatmentType::Transfer, vec![risk_treatments.treatment_outsource]).await.unwrap();

    RiskAnalysisService::sync_org_risk_treatment(&mut *tx, ra.clone(), vec![risk_treatments.treatment_guidelines, risk_treatments.treatment_training]).await.unwrap();

    ra
}


async fn create_scenario_it_grundschutz_check(tx: &mut PgConnection, risk_treatments: TestRiskTreatments) -> String {
    let ra = create_scenario_risk_treatment(&mut *tx, risk_treatments.clone()).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::RiskTreatment).await.unwrap();

    let full_assessment = ItGrundschutCheckService::full_assessment(&mut *tx, ra.clone()).await.unwrap();

    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.org.as_ref().unwrap().items[0].id,
                                                                 RiskAssessmentUpdateModel{
                                                                     status: ImplementationStatus::Full,
                                                                     evaluation: "Materiály týkajúce sa bezpečnostných pokynov sú teraz k dispozícii všetkým zamestnancom.".to_string()
                                                                 }).await.unwrap();
    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.org.as_ref().unwrap().items[1].id,
                                                                 RiskAssessmentUpdateModel{
                                                                     status: ImplementationStatus::None,
                                                                     evaluation: "Nepodarilo sa nám nájsť odborníka na bezpečnostné školenia.".to_string()
                                                                 }).await.unwrap();

    ItGrundschutCheckService::update_security_measure_assessment(&mut *tx, full_assessment.threats[0].items[0].id,
                                                                 RiskAssessmentUpdateModel{
                                                                     status: ImplementationStatus::Partial,
                                                                     evaluation: "Zamestnanci a priestory boli presunuté to čistejších miestností, ešte sme nezaobstarali čističku vzduchu.".to_string()
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

async fn create_scenario_completed(tx: &mut PgConnection, risk_treatments: TestRiskTreatments) -> String {
    let ra = create_scenario_it_grundschutz_check(&mut *tx, risk_treatments.clone()).await;
    RiskAnalysisService::complete_step(&mut *tx, ra.clone(), RiskAnalysisState::ItGrundschutzCheck).await.unwrap();

    ra
}
