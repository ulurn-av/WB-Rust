use axum::extract::{Path, Json};
use crate::order::models::Order;

pub async fn get_order_by_uid(Path(uid): Path<String>) {

}

pub async fn create_order(Json(payload) : Json<Order>) {

}

