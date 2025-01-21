use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let (db_conn, listener) = setting::get().await;

    tracing_subscriber::fmt::init();

    let routes = api::compose().with_state(db_conn);

    info!(
        "Server started at http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, routes).await.unwrap();
}
