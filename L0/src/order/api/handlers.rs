use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::debug_handler;
use axum::extract::{Path, Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde_json::json;
use crate::db::connection::Database;
use crate::order::models::{Order, Test};
use crate::order::services::OrderService;

#[debug_handler]
pub async fn get_order_by_uid(
    Path(uid): Path<String>,
    State(database): State<Arc<Mutex<Database>>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let database = database.lock().await;
    let service = OrderService::new(database.clone());

    match service.get_order_by_uid(&uid).await {
        Ok(response) => Ok(Json(response).into_response()),
        Err(err) => {
            let error_response = json!({
                "error": "Order not found or an error occurred",
                "details": err,
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }

}

// pub async fn create_order(
//     Json(payload) : Json<Order>,
//     State(database): State<Arc<Mutex<Database>>>
// ) -> Order {
// }

