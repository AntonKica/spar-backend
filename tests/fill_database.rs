use spar_backend::configuration::AppConfig;
use spar_backend::create_connection;
use spar_backend::enums::asset_enums::AssetType;
use spar_backend::enums::asset_enums::ProtectionNeeds;
use spar_backend::enums::risk_analysis_process_enums::ProcessStep;
use spar_backend::enums::step_2_threat_identification_enums::ThreatRelevance;
use spar_backend::enums::step_3_risk_classification_enums::{ThreatImpact, ThreatProbability};
use spar_backend::enums::step_4_risk_treatment_enums::RiskTransferType;
use spar_backend::model::asset_model::AssetCreateModel;
use spar_backend::model::security_measure_models::SecurityMeasureCreateModel;
use spar_backend::model::step_2_threat_identification_models::TourThreatReviewModel;
use spar_backend::model::step_3_risk_classification_models::TourRiskClassificationClassifyModel;
use spar_backend::model::step_4_risk_treatment_models::{RiskAcceptanceCreateModel, RiskAvoidanceCreateModel, RiskReductionCreateModel, RiskTransferCreateModel};
use spar_backend::model::threat_models::ThreatCreateModel;
use spar_backend::service::asset_service::AssetService;
use spar_backend::service::risk_analysis_process_service::RiskAnalysisProcessService;
use spar_backend::service::security_measure_service::SecurityMeasureService;
use spar_backend::service::step_2_threat_idenfication_service::Step2ThreatIdentificationService;
use spar_backend::service::step_3_risk_classification_service::Step3RiskClassificationService;
use spar_backend::service::step_4_risk_treatment_service::Step4RiskTreatmentService;
use spar_backend::service::threat_service::ThreatService;
use spar_backend::service::GeneralService;
use sqlx::PgConnection;

async fn clear_database(tx: &mut PgConnection) {
    sqlx::query(r#"DELETE FROM risk_treatment"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_acceptance"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_avoidance"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_transfer"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_reduction"#).execute(&mut *tx).await.unwrap();

    sqlx::query(r#"DELETE FROM risk_treatment_code"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_classification"#).execute(&mut *tx).await.unwrap();

    sqlx::query(r#"DELETE FROM tour_threat_list"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM rap_tour_list"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM risk_analysis_process"#).execute(&mut *tx).await.unwrap();

    sqlx::query(r#"DELETE FROM asset"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM threat WHERE code NOT SIMILAR TO 'G-\d\d'"#).execute(&mut *tx).await.unwrap();
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
    let switch = AssetService::create(&mut *tx, AssetCreateModel{
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
    
    let sth = ThreatService::create(&mut *tx, ThreatCreateModel {
        name: "first specific threat".to_string(),
        confidentiality_impaired: true,
        integrity_impaired: false,
        availability_impaired: true,
        description: "description".to_string(),
    }).await.unwrap();

    let sm = SecurityMeasureService::create(&mut tx, SecurityMeasureCreateModel{
        name: "prvé bezpečnostné opatrenie".to_owned(),
        description: "rutinné bezpečnostné opatrenie".to_owned(),
        confidentiality_protected: true,
        integrity_protected: false,
        availability_protected: true,
    }).await.unwrap();

    AssetService::assign_security_measure(&mut *tx, bp.clone(), sm).await.unwrap();

    let rap = RiskAnalysisProcessService::create(&mut *tx).await.unwrap();
    RiskAnalysisProcessService::set_tour(&mut *tx, rap.clone(), vec![bp.clone(), switch.clone()]).await.unwrap();
    RiskAnalysisProcessService::step_complete(&mut *tx, rap.clone(), ProcessStep::Step1SelectTour).await.unwrap();

    Step2ThreatIdentificationService::threat_review(&mut *tx, rap.clone(), bp.clone(), "G-01".to_owned(), TourThreatReviewModel {
        relevance: ThreatRelevance::Indirect,
        explanation: "Len tak".to_string(),
    }).await.unwrap();

    Step2ThreatIdentificationService::threat_review(&mut *tx, rap.clone(), bp.clone(), "G-02".to_owned(), TourThreatReviewModel {
        relevance: ThreatRelevance::Indirect,
        explanation: "Len tak".to_string(),
    }).await.unwrap();

    Step2ThreatIdentificationService::threat_review(&mut *tx, rap.clone(), bp.clone(), "G-03".to_owned(), TourThreatReviewModel {
        relevance: ThreatRelevance::Direct,
        explanation: "Len tak".to_string(),
    }).await.unwrap();

    Step2ThreatIdentificationService::threat_review(&mut *tx, rap.clone(), bp.clone(), sth.clone(), TourThreatReviewModel {
        relevance: ThreatRelevance::Direct,
        explanation: "Len tak".to_string(),
    }).await.unwrap();

    Step2ThreatIdentificationService::threat_review(&mut *tx, rap.clone(), switch.clone(), "G-01".to_owned(), TourThreatReviewModel {
        relevance: ThreatRelevance::Direct,
        explanation: "Len tak".to_string(),
    }).await.unwrap();

    Step2ThreatIdentificationService::threat_review(&mut *tx, rap.clone(), switch.clone(), sth.clone(), TourThreatReviewModel {
        relevance: ThreatRelevance::Direct,
        explanation: "Len tak".to_string(),
    }).await.unwrap();

    RiskAnalysisProcessService::step_complete(&mut *tx, rap.clone(), ProcessStep::Step2RelevantThreatIdentification).await.unwrap();


    Step3RiskClassificationService::threat_classify(&mut *tx, rap.clone(), bp.clone(), "G-01".to_owned(), TourRiskClassificationClassifyModel{
        probability: ThreatProbability::Medium,
        impact: ThreatImpact::Significant,
        evaluation: "ev bp g01".to_string(),
    }).await.unwrap();

    Step3RiskClassificationService::threat_classify(&mut *tx, rap.clone(), bp.clone(), "G-02".to_owned(), TourRiskClassificationClassifyModel{
        probability: ThreatProbability::VeryOften,
        impact: ThreatImpact::Limited,
        evaluation: "ev bp g02".to_string(),
    }).await.unwrap();

    Step3RiskClassificationService::threat_classify(&mut *tx, rap.clone(), bp.clone(), sth.clone(), TourRiskClassificationClassifyModel{
        probability: ThreatProbability::VeryOften,
        impact: ThreatImpact::LifeThreatening,
        evaluation: "ev bp thr".to_string(),
    }).await.unwrap();

    Step3RiskClassificationService::threat_classify(&mut *tx, rap.clone(), switch.clone(), "G-01".to_owned(), TourRiskClassificationClassifyModel{
        probability: ThreatProbability::Medium,
        impact: ThreatImpact::Limited,
        evaluation: "ev bp g01".to_string(),
    }).await.unwrap();

    Step3RiskClassificationService::threat_classify(&mut *tx, rap.clone(), switch.clone(), sth.clone(), TourRiskClassificationClassifyModel{
        probability: ThreatProbability::Often,
        impact: ThreatImpact::LifeThreatening,
        evaluation: "ev bp thr".to_string(),
    }).await.unwrap();

    RiskAnalysisProcessService::step_complete(&mut *tx, rap.clone(), ProcessStep::Step3RiskClassification).await.unwrap();

    let risk_accept = Step4RiskTreatmentService::risk_accept_with_create(&mut *tx, rap.clone(), bp.clone(), "G-01".to_owned(), RiskAcceptanceCreateModel{
        name: "Akceptovanie by default".to_owned(),
        explanation: "Riziko je zvládnuteľné".to_owned()
    }).await.unwrap();

    Step4RiskTreatmentService::risk_avoid_with_create(&mut *tx, rap.clone(), bp.clone(), "G-02".to_owned(), RiskAvoidanceCreateModel{
        name: "Vyhnutie by default".to_owned(),
        explanation: "Riziko nie je podstatne, vymazali sme zdrojov pôvodnej hrozby".to_owned()
    }).await.unwrap();

    Step4RiskTreatmentService::risk_transfer_with_create(&mut *tx, rap.clone(), bp.clone(), "G-03".to_owned(), RiskTransferCreateModel{
        name: "Outsourcovanie vyvoja na indianov".to_owned(),
        risk_transfer_type: RiskTransferType::Outsourcing,
        checklist: vec!["zabecpecit vhodne zmluvne podmienky".to_owned(), "zabezpecit kvalitne vyvojove štandardy".to_owned()],
        explanation: "IT v Indii to zvladne lepsie".to_owned()
    }).await.unwrap();

    Step4RiskTreatmentService::risk_reduce_with_create(&mut *tx, rap.clone(), bp.clone(), sth.clone(), RiskReductionCreateModel{
        name: "Bezpecnosnte opatrenie c.1".to_owned(),
        confidentiality_protected: true,
        integrity_protected: true,
        availability_protected: false,
        explanation: "Toto opatrenie pomoze znizit dopad na dôvernost".to_owned()
    }).await.unwrap();

    Step4RiskTreatmentService::risk_reduce_with_create(&mut *tx, rap.clone(), bp.clone(), sth.clone(), RiskReductionCreateModel{
        name: "Bezpecnosnte opatrenie c.2".to_owned(),
        confidentiality_protected: false,
        integrity_protected: true,
        availability_protected: true,
        explanation: "Toto opatrenie pomoze znizit dopad na dostupnost".to_owned()
    }).await.unwrap();

    let risk_reduce = Step4RiskTreatmentService::risk_reduce_with_create(&mut *tx, rap.clone(), bp.clone(), sth.clone(), RiskReductionCreateModel{
        name: "Bezpecnosnte opatrenie c.3".to_owned(),
        confidentiality_protected: false,
        integrity_protected: false,
        availability_protected: true,
        explanation: "Toto opatrenie pomoze vyrazne znizit dopad na dostupnost".to_owned()
    }).await.unwrap();

    Step4RiskTreatmentService::risk_accept(&mut *tx, rap.clone(), switch.clone(), "G-01".to_owned(), risk_accept.clone()).await.unwrap();
    Step4RiskTreatmentService::risk_reduce(&mut *tx, rap.clone(), switch.clone(), sth.clone(), vec![risk_reduce.clone()]).await.unwrap();
    Step4RiskTreatmentService::risk_reduce_with_create(&mut *tx, rap.clone(), switch.clone(), sth.clone(), RiskReductionCreateModel{
        name: "Bezpecnosnte opatrenie c.4".to_owned(),
        confidentiality_protected: false,
        integrity_protected: false,
        availability_protected: true,
        explanation: "Toto opatrenie pomoze vyrazne znizit dopad na dovernost".to_owned()
    }).await.unwrap();

    tx.commit().await.unwrap();
}