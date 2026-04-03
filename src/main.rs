use server::{api, database::init, get as get_setting};
use tracing::info;

#[tokio::main]
async fn main() {
    let (listener, app_state, site_init) = get_setting().await;

    init(site_init, &app_state.db).await.unwrap();

    tracing_subscriber::fmt::init();

    let routes = api::compose(app_state.clone()).with_state(app_state);

    info!(
        "Server started at http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, routes).await.unwrap();
}
