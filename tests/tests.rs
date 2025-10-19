use spar_backend::create_connection;
use spar_backend::enums::{BusinessProcessType, ModuleType};
use spar_backend::model::{set_responsible, ApplicationCreateModel, BusinessProcessCreateModel, BusinessProcessRoleCreateModel, ITSystemCreateModel, RoleCreateModel};

#[tokio::test]
async fn populate_db() {
    let db = create_connection().await;

    sqlx::query("DELETE FROM business_process__role").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM business_process").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM application").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM it_system").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM role").execute(&db).await.unwrap();

    ///
    let bp_product_development =
        BusinessProcessCreateModel {
            name: "vývoj produktov".to_owned(),
            description: "vývoj metrologických systémov".to_owned(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();

    let role_cto = RoleCreateModel {
        name: "vedúci technického oddelenia".to_owned(),
        description: "zodpovedá za SW, HW a QA vývojárov".to_owned(),
    }.create(&db).await.unwrap();
    let role_sw_developers = RoleCreateModel {
        name: "softvérový vývojár".to_owned(),
        description: "návrh, vývoj a testovanie softvérových komponentov".to_owned(),
    }.create(&db).await.unwrap();
    let role_hw_developers = RoleCreateModel {
        name: "hardvérový vývojár".to_owned(),
        description: "návrh, vývoj a testovanie hardvérových komponentov".to_owned(),
    }.create(&db).await.unwrap();
    let role_qa = RoleCreateModel {
        name: "kontrolór kvality".to_owned(),
        description: "zabezpečenie splnenia požiadaviek produktu".to_owned(),
    }.create(&db).await.unwrap();

    set_responsible(bp_product_development.clone(), role_cto.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_product_development.clone(),
        role_code: role_sw_developers.clone()
    }.assign(&db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_product_development.clone(),
        role_code: role_hw_developers.clone()
    }.assign(&db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_product_development.clone(),
        role_code: role_qa.clone()
    }.assign(&db).await.unwrap();


    ///
    let bp_production =
        BusinessProcessCreateModel {
            name: "výroba".to_owned(),
            description: "montáž a kalibrácia".to_owned(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();

    let role_production_leader = RoleCreateModel {
        name: "vedúci výroby".to_owned(),
        description: "zodpovedá za výrobu".to_owned(),
    }.create(&db).await.unwrap();
    let role_mechanic  = RoleCreateModel {
        name: "mechanik".to_owned(),
        description: "montér, kalibrator".to_owned(),
    }.create(&db).await.unwrap();

    set_responsible(bp_production.clone(), role_production_leader.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_production.clone(),
        role_code: role_mechanic.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_integration =
        BusinessProcessCreateModel {
            name: "integrácia".to_owned(),
            description: "inštalácia a servis".to_owned(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();
    let role_integration_leader = RoleCreateModel {
        name: "vedúci integrátorov".to_owned(),
        description: "zodpovedá za integrátorov".to_owned(),
    }.create(&db).await.unwrap();
    let role_integrator  = RoleCreateModel {
        name: "integrátor".to_owned(),
        description: "inštalatér a servis na lokalite".to_owned(),
    }.create(&db).await.unwrap();

    set_responsible(bp_integration.clone(), role_integration_leader.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_integration.clone(),
        role_code: role_integrator.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_sales =
        BusinessProcessCreateModel {
            name: "obchod a marketing".to_owned(),
            description: "obchod, marketing a predaj".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();

    let role_sales_manager = RoleCreateModel {
        name: "obchodný riaditeľ".to_owned(),
        description: "zodpovedá za obchod".to_owned(),
    }.create(&db).await.unwrap();
    let role_salesman = RoleCreateModel {
        name: "obchodník".to_owned(),
        description: "obchodník".to_owned(),
    }.create(&db).await.unwrap();

    set_responsible(bp_sales.clone(), role_sales_manager.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_sales.clone(),
        role_code: role_salesman.clone()
    }.assign(&db).await.unwrap();


    ///
    let bp_support =
        BusinessProcessCreateModel {
            name: "support".to_owned(),
            description: "podpora pre zákazníkov".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_support_lead = RoleCreateModel {
        name: "vedúci podpory".to_owned(),
        description: "zodpovedá za podporu".to_owned(),
    }.create(&db).await.unwrap();
    let role_support  = RoleCreateModel {
        name: "podpora".to_owned(),
        description: "podpora na linke".to_owned(),
    }.create(&db).await.unwrap();

    set_responsible(bp_support.clone(), role_support_lead.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_support.clone(),
        role_code: role_support.clone()
    }.assign(&db).await.unwrap();


    ///
    let bp_qa =
        BusinessProcessCreateModel {
            name: "riadenie kvality".to_owned(),
            description: "kontrola kvality".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_qa_manager = RoleCreateModel {
        name: "manažér kvality".to_owned(),
        description: "zodpovedá za kvalitu".to_owned(),
    }.create(&db).await.unwrap();
    set_responsible(bp_qa.clone(), role_qa_manager.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_qa.clone(),
        role_code: role_qa.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_hr =
        BusinessProcessCreateModel {
            name: "ľudské zdroje a administratíva".to_owned(),
            description: "personálne oddelenie a administratíva".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_office_manager  = RoleCreateModel {
        name: "office manažér".to_owned(),
        description: "office manažér".to_owned(),
    }.create(&db).await.unwrap();
    let role_assistant  = RoleCreateModel {
        name: "asistent".to_owned(),
        description: "asistent".to_owned(),
    }.create(&db).await.unwrap();
    let role_personalist  = RoleCreateModel {
        name: "personalista".to_owned(),
        description: "personalista".to_owned(),
    }.create(&db).await.unwrap();
    set_responsible(bp_hr.clone(), role_office_manager.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_hr.clone(),
        role_code: role_assistant.clone()
    }.assign(&db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_hr.clone(),
        role_code: role_personalist.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_finances =
        BusinessProcessCreateModel {
            name: "financie".to_owned(),
            description: "financie a nákupy".to_owned(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_cfo  = RoleCreateModel {
        name: "výkonný riaditeľ".to_owned(),
        description: "CFO".to_owned(),
    }.create(&db).await.unwrap();
    let role_accountant  = RoleCreateModel {
        name: "účtovník".to_owned(),
        description: "účtovník".to_owned(),
    }.create(&db).await.unwrap();
    set_responsible(bp_finances.clone(), role_cfo.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_finances.clone(),
        role_code: role_accountant.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_strategic_planning =
        BusinessProcessCreateModel {
            name: "Strategické plánovanie".to_owned(),
            description: "výhľad do budúcnosti".to_owned(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();
    let role_ceo  = RoleCreateModel {
        name: "generálny riaditeľ".to_owned(),
        description: "CEO".to_owned(),
    }.create(&db).await.unwrap();
    set_responsible(bp_strategic_planning.clone(), role_ceo.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_strategic_planning.clone(),
        role_code: role_ceo.clone()
    }.assign(&db).await.unwrap();
    // bp_human_resources.assign_role(&role_programmers, &db).await.unwrap();


    let role_everyone  = RoleCreateModel {
        name: "každý".to_owned(),
        description: "všetci zamestnanci".to_owned(),
    }.create(&db).await.unwrap();
    let role_it_services  = RoleCreateModel {
        name: "IT služby".to_owned(),
        description: "ľudia prevádzkujúci naše IT služby".to_owned(),
    }.create(&db).await.unwrap();
    let role_administrator  = RoleCreateModel {
        name: "administrátor".to_owned(),
        description: "ľudia prevádzkujúci naše IT služby".to_owned(),
    }.create(&db).await.unwrap();
    let role_production_department  = RoleCreateModel {
        name: "produkčné oddelenie".to_owned(),
        description: "ľudia prevádzkujúci a spravujúci produkčné prostredie".to_owned(),
    }.create(&db).await.unwrap();

    let app_its = ApplicationCreateModel {
        name: "Redmine".to_owned(),
        description: "ITS systém".to_owned(),
        module_type: ModuleType::APP_6_GENERAL_SOFTWARE.to_owned(),
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let app_chat = ApplicationCreateModel {
        name: "Rocket Chat".to_owned(),
        description: "komunikačný system".to_owned(),
        module_type: ModuleType::APP_6_GENERAL_SOFTWARE.to_owned(),
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let app_word_processor = ApplicationCreateModel {
        name: "Libre Office".to_owned(),
        description: "kancelársky balík".to_owned(),
        module_type: ModuleType::APP_1_1_OFFICE_PRODUCTS.to_owned(),
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();

    let it_system_router = ITSystemCreateModel {
        name: "externý router".to_owned(),
        description: "zabezpečuje komunikáciu medzi internetom a interným prostredím".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let it_system_external_firewall = ITSystemCreateModel {
        name: "firewall".to_owned(),
        description: "zabezpečuje bezpečnú role_administrator medzi internými systémami a internetom".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let it_system_internal_switch = ITSystemCreateModel {
        name: "interný switch".to_owned(),
        description: "zodpovedný za tok údajov medzi internou sieťou a internetom".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let it_system_print_server = ITSystemCreateModel {
        name: "tlačiareň".to_owned(),
        description: "kancelársky balík".to_owned(),
        module_type: ModuleType::SYS_1_2_2_WINDOWS_SERVER_2012.to_owned(),
        count: 1,
        application_user: role_everyone.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let it_system_virtual_server = ITSystemCreateModel {
        name: "virtuálny server".to_owned(),
        description: "hosťuje vyše 20 virtuálnych serverov za použitia virtualizačného SW".to_owned(),
        module_type: ModuleType::SYS_1_3_LINUX_AND_UNIX_SERVERS.to_owned(),
        count: 2,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let it_system_production_router = ITSystemCreateModel {
        name: "production router".to_owned(),
        description: "zabezpečuje komunikáciu medzi interným prostredím a produkčným prostredím".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 2,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let it_system_production_switch = ITSystemCreateModel {
        name: "production switch".to_owned(),
        description: "zodpovedný za tok údajov medzi produkčným prostredím a internou sieťou".to_owned(),
        module_type: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES.to_owned(),
        count: 1,
        application_user: role_administrator.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
    let it_system_production_server = ITSystemCreateModel {
        name: "centrálny produkčný server".to_owned(),
        description: "spracováva všetky dáta".to_owned(),
        module_type: ModuleType::SYS_1_3_LINUX_AND_UNIX_SERVERS.to_owned(),
        count: 2,
        application_user: role_production_department.clone(),
        responsible: role_administrator.clone(),
    }.create(&db).await.unwrap();
}