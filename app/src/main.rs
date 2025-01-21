use database::init;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let (db_conn, listener, site_init) = setting::get().await;

    init(site_init, &db_conn).await.unwrap();

    tracing_subscriber::fmt::init();

    let routes = api::compose().with_state(db_conn);

    info!(
        "Server started at http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, routes).await.unwrap();
}
