use spar_backend::configuration::AppConfig;
use spar_backend::create_connection;
use spar_backend::enums::{BusinessProcessType, ModuleType, ProtectionNeeds};
use spar_backend::model::{ApplicationCreateModel, AssetCreateModel, BusinessProcessCreateModel, ITSystemCreateModel, RiskAnalysisProcessCreateModel, RoleCreateModel, TOURElementaryThreatUpdateModel, TOURSpecificThreatCreateModel};
use spar_backend::service::application_service::ApplicationService;
use spar_backend::service::asset_service::AssetService;
use spar_backend::service::business_process_service::BusinessProcessService;
use spar_backend::service::GeneralService;
use spar_backend::service::it_system_service::ITSystemService;
use spar_backend::service::risk_analysis_process_service::RiskAnalysisProcessService;
use spar_backend::service::role_service::RoleService;

#[tokio::test]
async fn create_dummy_assets() {
    let config = AppConfig::from_env();
    let db = create_connection(&config).await;

    sqlx::query("DELETE FROM tour_elementary_threat_risk_classification").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM tour_specific_threat_risk_classification").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM tour_elementary_threat").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM tour_specific_threat").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM tour_specific_threat_overview").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM target_object_under_review").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM risk_analysis_process").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM asset").execute(&db).await.unwrap();

    let mut tx = db.begin().await.unwrap();

    let virt_server = AssetService::create(&mut tx, AssetCreateModel {
        name: "dummy virtualizačný server".to_owned(),
        description: "bežia tu dev aplikáie".to_owned(),
        confidentiality_protection_needs: ProtectionNeeds::High,
        integrity_protection_needs: ProtectionNeeds::High,
        availability_protection_needs: ProtectionNeeds::High,
        responsible: "IT administrátor".to_owned(),
    }).await.unwrap();

    let switch = AssetService::create(&mut tx, AssetCreateModel {
        name: "dummy switch".to_owned(),
        description: "rozdeľuje sieť na virtuálne podsiete".to_owned(),
        responsible: "sieťový administrátor".to_owned(),
        confidentiality_protection_needs: ProtectionNeeds::Normal,
        integrity_protection_needs: ProtectionNeeds::Normal,
        availability_protection_needs: ProtectionNeeds::High,
    }).await.unwrap();

    let database = AssetService::create(&mut tx, AssetCreateModel {
        name: "dummy databáza".to_owned(),
        description: "sídlia tu všetky dáta".to_owned(),
        responsible: "IT administrátor".to_owned(),
        confidentiality_protection_needs: ProtectionNeeds::VeryHigh,
        integrity_protection_needs: ProtectionNeeds::VeryHigh,
        availability_protection_needs: ProtectionNeeds::VeryHigh,
    }).await.unwrap();


    let rap= RiskAnalysisProcessService::create(
        &mut tx,
        RiskAnalysisProcessCreateModel {
            target_objects_under_review: vec![virt_server.clone(), switch.clone(), database.clone()]
        }
    ).await.unwrap();
    tx.commit().await.unwrap();

    for tour in vec![virt_server, switch, database] {
        let mut etl = RiskAnalysisProcessService::get_elementary_threat_list(&db, rap.clone(), tour.clone()).await.unwrap();
        etl[0].relevance = 1;
        etl[1].relevance = 2;
        for et in etl.iter_mut() {
            et.reviewed = true;
        }

        let update = etl.iter().map(|et| TOURElementaryThreatUpdateModel {
            elementary_threat_code: et.elementary_threat_code.clone(),
            relevance: et.relevance,
            comment: et.comment.clone(),
            reviewed: et.reviewed
        }).collect();
        RiskAnalysisProcessService::update_elementary_threat_list(&db, rap.clone(), tour.clone(), update).await.unwrap();

        RiskAnalysisProcessService::create_specific_threat(&db, rap.clone(), tour.clone(), TOURSpecificThreatCreateModel {
            name: "Dummy specific threat".to_owned(),
            description: "just a dummy specific threat".to_owned(),
            confidentiality_impaired: true,
            integrity_impaired: true,
            availability_impaired: false
        }).await.unwrap();

        RiskAnalysisProcessService::specific_threat_overview_set_reviewed(&db, rap.clone(), tour.clone(), true).await.unwrap();
    }


    let mut tx = db.begin().await.unwrap();
    RiskAnalysisProcessService::step_1_threat_overview_finish(&mut tx, rap.clone()).await.unwrap();
    tx.commit().await.unwrap();
}
#[tokio::test]
async fn populate_db() {
    let config = AppConfig::from_env();
    let db = create_connection(&config).await;

    sqlx::query("DELETE FROM business_process__application").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM business_process__role").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM business_process").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM application").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM it_system").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM role").execute(&db).await.unwrap();

    let mut tx = db.begin().await.unwrap();
    ///
    let bp_product_development =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "vývoj produktov".to_owned(),
            description: "vývoj metrologických systémov".to_owned(),
            process_type: BusinessProcessType::PRIMARY,
        }).await.unwrap();

    let role_cto = RoleService::create(&mut tx, RoleCreateModel {
        name: "vedúci technického oddelenia".to_owned(),
        description: "zodpovedá za SW, HW a QA vývojárov".to_owned(),
    }).await.unwrap();
    let role_sw_developers = RoleService::create(&mut tx, RoleCreateModel {
        name: "softvérový vývojár".to_owned(),
        description: "návrh, vývoj a testovanie softvérových komponentov".to_owned(),
    }).await.unwrap();
    let role_hw_developers = RoleService::create(&mut tx, RoleCreateModel {
        name: "hardvérový vývojár".to_owned(),
        description: "návrh, vývoj a testovanie hardvérových komponentov".to_owned(),
    }).await.unwrap();
    let role_qa = RoleService::create(&mut tx, RoleCreateModel {
        name: "kontrolór kvality".to_owned(),
        description: "zabezpečenie splnenia požiadaviek produktu".to_owned(),
    }).await.unwrap();

    BusinessProcessService::set_responsible(&mut tx, bp_product_development.clone(), role_cto.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_product_development.clone(), role_sw_developers.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_product_development.clone(), role_hw_developers.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_product_development.clone(), role_qa.clone()).await.unwrap();


    ///
    let bp_production = BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
        name: "výroba".to_owned(),
        description: "montáž a kalibrácia".to_owned(),
        process_type: BusinessProcessType::PRIMARY,
    }).await.unwrap();

    let role_production_leader = RoleService::create(&mut tx, RoleCreateModel {
        name: "vedúci výroby".to_owned(),
        description: "zodpovedá za výrobu".to_owned(),
    }).await.unwrap();
    let role_mechanic  = RoleService::create(&mut tx, RoleCreateModel {
        name: "mechanik".to_owned(),
        description: "montér, kalibrator".to_owned(),
    }).await.unwrap();

    BusinessProcessService::set_responsible(&mut tx, bp_production.clone(), role_production_leader.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_production.clone(), role_mechanic.clone()).await.unwrap();

    ///
    let bp_integration =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "integrácia".to_owned(),
            description: "inštalácia a servis".to_owned(),
            process_type: BusinessProcessType::PRIMARY,
        }).await.unwrap();
    let role_integration_leader = RoleService::create(&mut tx, RoleCreateModel {
        name: "vedúci integrátorov".to_owned(),
        description: "zodpovedá za integrátorov".to_owned(),
    }).await.unwrap();
    let role_integrator  = RoleService::create(&mut tx, RoleCreateModel {
        name: "integrátor".to_owned(),
        description: "inštalatér a servis na lokalite".to_owned(),
    }).await.unwrap();

    BusinessProcessService::set_responsible(&mut tx, bp_integration.clone(), role_integration_leader.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_integration.clone(), role_integrator.clone()).await.unwrap();

    ///
    let bp_sales =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "obchod a marketing".to_owned(),
            description: "obchod, marketing a predaj".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }).await.unwrap();

    let role_sales_manager = RoleService::create(&mut tx, RoleCreateModel {
        name: "obchodný riaditeľ".to_owned(),
        description: "zodpovedá za obchod".to_owned(),
    }).await.unwrap();
    let role_salesman = RoleService::create(&mut tx, RoleCreateModel {
        name: "obchodník".to_owned(),
        description: "obchodník".to_owned(),
    }).await.unwrap();

    BusinessProcessService::set_responsible(&mut tx, bp_sales.clone(), role_sales_manager.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_sales.clone(), role_salesman.clone()).await.unwrap();


    ///
    let bp_support =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "support".to_owned(),
            description: "podpora pre zákazníkov".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }).await.unwrap();
    let role_support_lead = RoleService::create(&mut tx, RoleCreateModel {
        name: "vedúci podpory".to_owned(),
        description: "zodpovedá za podporu".to_owned(),
    }).await.unwrap();
    let role_support  = RoleService::create(&mut tx, RoleCreateModel {
        name: "podpora".to_owned(),
        description: "podpora na linke".to_owned(),
    }).await.unwrap();

    BusinessProcessService::set_responsible(&mut tx, bp_support.clone(), role_support_lead.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_support.clone(), role_support.clone()).await.unwrap();


    ///
    let bp_qa =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "riadenie kvality".to_owned(),
            description: "kontrola kvality".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }).await.unwrap();
    let role_qa_manager = RoleService::create(&mut tx, RoleCreateModel {
        name: "manažér kvality".to_owned(),
        description: "zodpovedá za kvalitu".to_owned(),
    }).await.unwrap();
    BusinessProcessService::set_responsible(&mut tx, bp_qa.clone(), role_qa_manager.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_qa.clone(), role_qa.clone()).await.unwrap();

    ///
    let bp_hr =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "ľudské zdroje a administratíva".to_owned(),
            description: "personálne oddelenie a administratíva".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }).await.unwrap();
    let role_office_manager  = RoleService::create(&mut tx, RoleCreateModel {
        name: "office manažér".to_owned(),
        description: "office manažér".to_owned(),
    }).await.unwrap();
    let role_assistant  = RoleService::create(&mut tx, RoleCreateModel {
        name: "asistent".to_owned(),
        description: "asistent".to_owned(),
    }).await.unwrap();
    let role_personalist  = RoleService::create(&mut tx, RoleCreateModel {
        name: "personalista".to_owned(),
        description: "personalista".to_owned(),
    }).await.unwrap();
    BusinessProcessService::set_responsible(&mut tx, bp_hr.clone(), role_office_manager.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_hr.clone(), role_assistant.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_hr.clone(), role_personalist.clone()).await.unwrap();

    ///
    let bp_finances =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "financie".to_owned(),
            description: "financie a nákupy".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }).await.unwrap();
    let role_cfo  = RoleService::create(&mut tx, RoleCreateModel {
        name: "výkonný riaditeľ".to_owned(),
        description: "CFO".to_owned(),
    }).await.unwrap();
    let role_accountant  = RoleService::create(&mut tx, RoleCreateModel {
        name: "účtovník".to_owned(),
        description: "účtovník".to_owned(),
    }).await.unwrap();
    BusinessProcessService::set_responsible(&mut tx, bp_finances.clone(), role_cfo.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_finances.clone(), role_accountant.clone()).await.unwrap();

    ///
    let bp_strategic_planning =
        BusinessProcessService::create(&mut tx, BusinessProcessCreateModel {
            name: "Strategické plánovanie".to_owned(),
            description: "výhľad do budúcnosti".to_owned(),
            process_type: BusinessProcessType::PRIMARY,
        }).await.unwrap();
    let role_ceo  = RoleService::create(&mut tx, RoleCreateModel {
        name: "generálny riaditeľ".to_owned(),
        description: "CEO".to_owned(),
    }).await.unwrap();
    BusinessProcessService::set_responsible(&mut tx, bp_strategic_planning.clone(), role_ceo.clone()).await.unwrap();
    BusinessProcessService::assign_role(&mut tx, bp_strategic_planning.clone(), role_ceo.clone()).await.unwrap();
    // bp_human_resources.assign_role(&role_programmers, &mut tx).await.unwrap();


    let role_everyone  = RoleService::create(&mut tx, RoleCreateModel {
        name: "každý".to_owned(),
        description: "všetci zamestnanci".to_owned(),
    }).await.unwrap();
    let role_it_services  = RoleService::create(&mut tx, RoleCreateModel {
        name: "IT služby".to_owned(),
        description: "ľudia prevádzkujúci naše IT služby".to_owned(),
    }).await.unwrap();
    let role_administrator  = RoleService::create(&mut tx, RoleCreateModel {
        name: "administrátor".to_owned(),
        description: "ľudia prevádzkujúci naše IT služby".to_owned(),
    }).await.unwrap();
    let role_production_department  = RoleService::create(&mut tx, RoleCreateModel {
        name: "produkčné oddelenie".to_owned(),
        description: "ľudia prevádzkujúci a spravujúci produkčné prostredie".to_owned(),
    }).await.unwrap();

    let app_its = ApplicationService::create(&mut tx, ApplicationCreateModel {
        name: "Redmine".to_owned(),
        description: "ITS systém".to_owned(),
        module_type: ModuleType::APP_6_GENERAL_SOFTWARE.to_owned(),
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let app_chat = ApplicationService::create(&mut tx, ApplicationCreateModel {
        name: "Rocket Chat".to_owned(),
        description: "komunikačný system".to_owned(),
        module_type: ModuleType::APP_6_GENERAL_SOFTWARE.to_owned(),
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let app_word_processor = ApplicationService::create(&mut tx, ApplicationCreateModel {
        name: "Libre Office".to_owned(),
        description: "kancelársky balík".to_owned(),
        module_type: ModuleType::APP_1_1_OFFICE_PRODUCTS.to_owned(),
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();

    BusinessProcessService::assign_application(&mut tx, bp_product_development.clone(), app_its.clone()).await.unwrap();
    BusinessProcessService::assign_application(&mut tx, bp_product_development.clone(), app_chat.clone()).await.unwrap();
    BusinessProcessService::assign_application(&mut tx, bp_product_development.clone(), app_chat.clone()).await.unwrap();

    BusinessProcessService::assign_application(&mut tx, bp_production.clone(), app_its.clone()).await.unwrap();
    BusinessProcessService::assign_application(&mut tx, bp_production.clone(), app_chat.clone()).await.unwrap();

    for bp in vec![bp_support, bp_hr, bp_sales] {
        BusinessProcessService::assign_application(&mut tx, bp.clone(), app_chat.clone()).await.unwrap();
        BusinessProcessService::assign_application(&mut tx, bp.clone(), app_word_processor.clone()).await.unwrap();
        BusinessProcessService::assign_application(&mut tx, bp.clone(), app_its.clone()).await.unwrap();
    }

    let it_system_router = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "externý router".to_owned(),
        description: "zabezpečuje komunikáciu medzi internetom a interným prostredím".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let it_system_external_firewall = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "firewall".to_owned(),
        description: "zabezpečuje bezpečnú role_administrator medzi internými systémami a internetom".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let it_system_internal_switch = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "interný switch".to_owned(),
        description: "zodpovedný za tok údajov medzi internou sieťou a internetom".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let it_system_print_server = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "tlačiareň".to_owned(),
        description: "kancelársky balík".to_owned(),
        module_type: ModuleType::SYS_1_2_2_WINDOWS_SERVER_2012.to_owned(),
        count: 1,
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let it_system_virtual_server = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "virtuálny server".to_owned(),
        description: "hosťuje vyše 20 virtuálnych serverov za použitia virtualizačného SW".to_owned(),
        module_type: ModuleType::SYS_1_3_LINUX_AND_UNIX_SERVERS.to_owned(),
        count: 2,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let it_system_production_router = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "production router".to_owned(),
        description: "zabezpečuje komunikáciu medzi interným prostredím a produkčným prostredím".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 2,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let it_system_production_switch = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "production switch".to_owned(),
        description: "zodpovedný za tok údajov medzi produkčným prostredím a internou sieťou".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();
    let it_system_production_server = ITSystemService::create(&mut tx, ITSystemCreateModel {
        name: "centrálny produkčný server".to_owned(),
        description: "spracováva všetky dáta".to_owned(),
        module_type: ModuleType::SYS_1_3_LINUX_AND_UNIX_SERVERS.to_owned(),
        count: 2,
        application_user: role_production_department.clone(),
        responsible: role_administrator.clone(),
    }).await.unwrap();

    tx.commit().await.unwrap();
}