use spar_backend::{create_connection, BusinessProcessModel, StaffModel};
use sqlx::{Pool, Postgres};

#[tokio::test]
async fn populate_db() {
    let db = create_connection().await;

    sqlx::query("DELETE FROM business_process__staff").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM business_process").execute(&db).await.unwrap();
    sqlx::query("DELETE FROM staff").execute(&db).await.unwrap();

    let bp_human_resources =
        BusinessProcessModel {
            code: "BP-0001".to_string(),
            name: "ľudské zdroje".to_string(),
            description: "správa zamestnancov, ekonomické oddelenie".to_string(),
            process_type: "podporný".to_string(),
            responsible: "vedúci oddelenia ľudských zdrojov".to_string(),
        };
    bp_human_resources.create(&db).await.unwrap();

    let staff_programmers = StaffModel {
        code: "STF-0001".to_string(),
        name: "programátori".to_string(),
        description: "programátori".to_string(),
    };
    staff_programmers.create(&db).await.unwrap();

    bp_human_resources.assign_staff(&staff_programmers, &db).await.unwrap();
}
