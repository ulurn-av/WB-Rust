mod order;
mod db;

use std::sync::Arc;
use axum::Router;
use order::api::routes::order_api;
use dotenv::dotenv;

pub async fn run() {
    dotenv().ok();

    let database = Arc::new(db::connection::Database::new()
        .await
        .unwrap()
    );

    let app = Router::new()
        .nest("/api/order", order_api(Arc::clone(&database)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("🔥Failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}