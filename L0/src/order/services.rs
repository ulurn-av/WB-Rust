use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::db::connection::Database;
// use crate::order::models::{Delivery, Item, Order, Payment};
use crate::order::models::{Delivery, Item, Order, Payment};

pub struct OrderService {
    pub database: Arc<Database>,
}

impl OrderService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    // pub async fn create_order(&self, order: Order) -> Result<Order, String> {
    //
    // }

    pub async fn get_order_by_uid(&self, order_uid: &str) -> Result<(Order, Payment, Delivery, Vec<Item>), String> {
        let order = sqlx::query_as!(
            Order,
            "SELECT * FROM orders WHERE order_uid = $1",
            order_uid
        )
        .fetch_one(&self.database.pool)
        .await.unwrap();

        let payment = sqlx::query_as!(
            Payment,
            r#"SELECT payment_id, order_id, transaction, request_id, currency,
             provider, amount::float as amount, bank, delivery_cost::float as delivery_cost,
             goods_total::float as goods_total, custom_fee::float as custom_fee
             FROM payments
             WHERE order_id = $1
             "#,
            order.order_id
        )
        .fetch_one(&self.database.pool)
        .await.unwrap();

        let delivery = sqlx::query_as!(
            Delivery,
            "SELECT * FROM delivery WHERE order_id = $1",
            order.order_id
        )
        .fetch_one(&self.database.pool)
        .await.unwrap();

        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT item_id, order_id, chrt_id, track_number, price::float AS price,
                rid, name, sale::float AS sale, size, total_price::float AS total_price,
                nm_id, brand, status
            FROM items
            WHERE order_id = $1
            "#,
            order.order_id
        )
        .fetch_all(&self.database.pool)
        .await.unwrap();

        Ok((order, payment, delivery, items))
    }
}