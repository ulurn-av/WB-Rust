use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::db::connection::Database;
use crate::order::services::OrderService;

#[debug_handler]
pub async fn get_order_by_uid(
    Path(order_uid): Path<String>,
    State(database): State<Arc<Database>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let service = OrderService::new(database.clone());

    match service.get_order_by_uid(&order_uid).await {
        Ok(full_order) => {
            let json_response = serde_json::json!({
                "order_uid": full_order.0.order_uid,
                "track_number": full_order.0.track_number,
                "entry": full_order.0.entry,
                "delivery": full_order.2,
                "payment": full_order.1,
                "items": full_order.3,
                "locale": full_order.0.locale,
                "customer_id": full_order.0.customer_id,
                "delivery_service": full_order.0.delivery_service,
                "shardkey": full_order.0.shardkey,
                "sm_id": full_order.0.sm_id,
                "oof_shard": full_order.0.oof_shard
            });
            Ok(Json(json_response))
        },
        Err(err) => {
            let error_response = json!({
                "error": "Order not found or an error occurred",
                "details": err,
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }

}

/*
#[debug_handler]
pub async fn create_order(
    State(database): State<Arc<Mutex<Database>>>,
    Json(payload) : Json<Order>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let database = database.lock().await;
    let service = OrderService::new(database.clone());
    
    match service.get_order_by_uid(&"1").await {
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
*/
