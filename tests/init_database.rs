use sqlx::PgConnection;
use spar_backend::configuration::AppConfig;
use spar_backend::create_connection;
use spar_backend::model::asset_model::AssetModelCreate;
use spar_backend::service::asset_service::AssetService;
use spar_backend::service::GeneralService;

async fn clear_database(tx: &mut PgConnection) {
    sqlx::query(r#"DELETE FROM risk_analysis_asset"#).execute(&mut *tx).await.unwrap();
    sqlx::query(r#"DELETE FROM asset"#).execute(&mut *tx).await.unwrap();
}
#[tokio::test]
async fn create_assets() {
    let config = AppConfig::from_env();
    let db = create_connection(&config).await;
    let mut tx = db.begin().await.unwrap();

    clear_database(&mut *tx).await;

    AssetService::create(&mut *tx, AssetModelCreate {
        name: "Workplace laptop".to_string(),
        description: "General laptop for employees enabling remote work".to_string(),
        module: "SYS-3-1".to_string(),
    }).await.unwrap();
    AssetService::create(&mut *tx, AssetModelCreate {
        name: "Administrator Laptop".to_string(),
        description: "Administrator laptop for server admins for remote server management".to_string(),
        module: "SYS-3-1".to_string(),
    }).await.unwrap();

    AssetService::create(&mut *tx, AssetModelCreate {
        name: "All the employees".to_string(),
        description: "All the employees of our organization".to_string(),
        module: "ORP-2".to_string(),
    }).await.unwrap();
    tx.commit().await.unwrap();
}
