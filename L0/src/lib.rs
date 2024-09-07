mod order;

use axum::Router;
use order::api::routes;

pub async fn run() {
    let app = Router::new()
        .nest("/api/order", routes::order_api());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}