use auth_service::configure_postgresql;
use auth_service::services::MockEmailClient;
use auth_service::services::PostgresUserStore;
use auth_service::services::RedisBannedTokenStore;
use auth_service::services::RedisTwoFACodeStore;
use auth_service::utils::constants::REDIS_HOST_NAME;
use auth_service::AppState;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
    let user_store = PostgresUserStore::new(pg_pool);

    let redis_client = auth_service::get_redis_client(REDIS_HOST_NAME.to_string()).unwrap();
    let banned_token_store = RedisBannedTokenStore::new(redis_client.clone());

    let two_fa_code_store = RedisTwoFACodeStore::new(redis_client);

    let email_client = MockEmailClient;
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
