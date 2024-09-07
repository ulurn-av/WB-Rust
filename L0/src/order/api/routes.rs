use axum::Router;
use axum::routing::{get, post};
use crate::order::api::handlers;

pub fn order_api() -> Router {
    Router::new()
        .route("/create", post(handlers::create_order))
        .route("/:order_uid", get(handlers::get_order_by_uid))
}