use auth_service::services::HashmapUserStore;
use auth_service::services::HashsetBannedTokenStore;
use auth_service::AppState;
use auth_service::Application;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::default();
    let banned_token_store = HashsetBannedTokenStore::default();
    let app_state = AppState::new(user_store, banned_token_store);

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
