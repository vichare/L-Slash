use anyhow::Result;
use l_slash::app;
use l_slash::services::user::generate_admin_user;
use l_slash::state::AppState;
use l_slash::storage::sled_store::SledStore;

#[tokio::main]
async fn main() -> Result<()> {
    let sled_store = SledStore::new("sled_data")?;
    sled_store.insert_user(&generate_admin_user())?;
    let is_release = !cfg!(debug_assertions);
    let state = AppState {
        sled_store,
        cookie_secure: is_release,
    };

    let app = app::build_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8090").await?;
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}
