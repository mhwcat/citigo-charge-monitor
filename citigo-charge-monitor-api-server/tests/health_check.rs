use citigo_charge_monitor_api_server::{config, run};
use sqlx::MySqlPool;

async fn spawn_app() {
    let config = config::Config::default();
    let db_connection_pool = MySqlPool::connect(&config.database_url)
        .await
        .expect("Failed connecting to db");
    let redis_connection_pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed connecting to Redis");

    let server = run(
        &config.api_base_addr,
        db_connection_pool,
        redis_connection_pool,
    )
    .expect("Failed to spawn app");

    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/api/health")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
