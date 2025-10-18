use spar_backend::create_connection;
use spar_backend::model::{set_responsible, BusinessProcessCreateModel, BusinessProcessRoleCreateModel, BusinessProcessType, RoleCreateModel};

#[tokio::test]
async fn populate_db() {
    let db = create_connection().await;

    sqlx::query("DELETE FROM business_process__role").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM business_process").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM role").execute(&db).await.unwrap();

    ///
    let bp_product_development =
        BusinessProcessCreateModel {
            name: "vývoj produktov".to_string(),
            description: "vývoj metrologických systémov".to_string(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();

    let role_cto = RoleCreateModel {
        name: "vedúci technického oddelenia".to_string(),
        description: "zodpovedá za SW, HW a QA vývojárov".to_string(),
    }.create(&db).await.unwrap();
    let role_sw_developers = RoleCreateModel {
        name: "softvérový vývojár".to_string(),
        description: "návrh, vývoj a testovanie softvérových komponentov".to_string(),
    }.create(&db).await.unwrap();
    let role_hw_developers = RoleCreateModel {
        name: "hardvérový vývojár".to_string(),
        description: "návrh, vývoj a testovanie hardvérových komponentov".to_string(),
    }.create(&db).await.unwrap();
    let role_qa = RoleCreateModel {
        name: "kontrolór kvality".to_string(),
        description: "zabezpečenie splnenia požiadaviek produktu".to_string(),
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
            name: "výroba".to_string(),
            description: "montáž a kalibrácia".to_string(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();

    let role_production_leader = RoleCreateModel {
        name: "vedúci výroby".to_string(),
        description: "zodpovedá za výrobu".to_string(),
    }.create(&db).await.unwrap();
    let role_mechanic  = RoleCreateModel {
        name: "mechanik".to_string(),
        description: "montér, kalibrator".to_string(),
    }.create(&db).await.unwrap();

    set_responsible(bp_production.clone(), role_production_leader.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_production.clone(),
        role_code: role_mechanic.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_integration =
        BusinessProcessCreateModel {
            name: "integrácia".to_string(),
            description: "inštalácia a servis".to_string(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();
    let role_integration_leader = RoleCreateModel {
        name: "vedúci integrátorov".to_string(),
        description: "zodpovedá za integrátorov".to_string(),
    }.create(&db).await.unwrap();
    let role_integrator  = RoleCreateModel {
        name: "integrátor".to_string(),
        description: "inštalatér a servis na lokalite".to_string(),
    }.create(&db).await.unwrap();

    set_responsible(bp_integration.clone(), role_integration_leader.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_integration.clone(),
        role_code: role_integrator.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_sales =
        BusinessProcessCreateModel {
            name: "obchod a marketing".to_string(),
            description: "obchod, marketing a predaj".to_string(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();

    let role_sales_manager = RoleCreateModel {
        name: "obchodný riaditeľ".to_string(),
        description: "zodpovedá za obchod".to_string(),
    }.create(&db).await.unwrap();
    let role_salesman = RoleCreateModel {
        name: "obchodník".to_string(),
        description: "obchodník".to_string(),
    }.create(&db).await.unwrap();

    set_responsible(bp_sales.clone(), role_sales_manager.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_sales.clone(),
        role_code: role_salesman.clone()
    }.assign(&db).await.unwrap();


    ///
    let bp_support =
        BusinessProcessCreateModel {
            name: "support".to_string(),
            description: "podpora pre zákazníkov".to_string(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_support_lead = RoleCreateModel {
        name: "vedúci podpory".to_string(),
        description: "zodpovedá za podporu".to_string(),
    }.create(&db).await.unwrap();
    let role_support  = RoleCreateModel {
        name: "podpora".to_string(),
        description: "podpora na linke".to_string(),
    }.create(&db).await.unwrap();

    set_responsible(bp_support.clone(), role_support_lead.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_support.clone(),
        role_code: role_support.clone()
    }.assign(&db).await.unwrap();


    ///
    let bp_qa =
        BusinessProcessCreateModel {
            name: "riadenie kvality".to_string(),
            description: "kontrola kvality".to_string(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_qa_manager = RoleCreateModel {
        name: "manažér kvality".to_string(),
        description: "zodpovedá za kvalitu".to_string(),
    }.create(&db).await.unwrap();
    set_responsible(bp_qa.clone(), role_qa_manager.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_qa.clone(),
        role_code: role_qa.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_hr =
        BusinessProcessCreateModel {
            name: "ľudské zdroje a administratíva".to_string(),
            description: "personálne oddelenie a administratíva".to_string(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_office_manager  = RoleCreateModel {
        name: "office manažér".to_string(),
        description: "office manažér".to_string(),
    }.create(&db).await.unwrap();
    let role_assistant  = RoleCreateModel {
        name: "asistent".to_string(),
        description: "asistent".to_string(),
    }.create(&db).await.unwrap();
    let role_personalist  = RoleCreateModel {
        name: "personalista".to_string(),
        description: "personalista".to_string(),
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
            name: "financie".to_string(),
            description: "financie a nákupy".to_string(),
            process_type: BusinessProcessType::SUPPORT,
        }.create(&db).await.unwrap();
    let role_cfo  = RoleCreateModel {
        name: "výkonný riaditeľ".to_string(),
        description: "CFO".to_string(),
    }.create(&db).await.unwrap();
    let role_accountant  = RoleCreateModel {
        name: "účtovník".to_string(),
        description: "účtovník".to_string(),
    }.create(&db).await.unwrap();
    set_responsible(bp_finances.clone(), role_cfo.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_finances.clone(),
        role_code: role_accountant.clone()
    }.assign(&db).await.unwrap();

    ///
    let bp_strategic_planning =
        BusinessProcessCreateModel {
            name: "Strategické plánovanie".to_string(),
            description: "výhľad do budúcnosti".to_string(),
            process_type: BusinessProcessType::PRIMARY,
        }.create(&db).await.unwrap();
    let role_ceo  = RoleCreateModel {
        name: "generálny riaditeľ".to_string(),
        description: "CEO".to_string(),
    }.create(&db).await.unwrap();
    set_responsible(bp_strategic_planning.clone(), role_ceo.clone(), &db).await.unwrap();
    BusinessProcessRoleCreateModel{
        business_process_code: bp_strategic_planning.clone(),
        role_code: role_ceo.clone()
    }.assign(&db).await.unwrap();
    // bp_human_resources.assign_role(&role_programmers, &db).await.unwrap();
}