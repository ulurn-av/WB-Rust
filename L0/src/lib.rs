mod order;
mod db;

use std::sync::{Arc, Mutex};
use axum::Router;
use axum::routing::get;
use dotenv::dotenv;
use sqlx::PgPool;

pub async fn run() {
    dotenv().ok();

    let database = Arc::new(Mutex::new(db::connection::Database::new()
        .await
        .unwrap()
    ));

    let app = Router::new()
        .nest("/api/order", order::api::routes::order_api(Arc::clone(&database)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("🔥Failed to bind to address");
    axum::serve(listener, app).await.unwrap();
}