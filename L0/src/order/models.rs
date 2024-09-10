use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

// Модель для таблицы orders
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub order_id: i32,
    pub order_uid: String,
    pub track_number: String,
    pub entry: String,
    pub locale: String,
    pub customer_id: String,
    pub delivery_service: String,
    pub shardkey: String,
    pub sm_id: i32,
    pub oof_shard: String,
}

// Модель для таблицы payments
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub payment_id: i32,
    pub order_id: i32,  // Внешний ключ на orders
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: Option<f64>,
    pub bank: String,
    pub delivery_cost: Option<f64>,
    pub goods_total: Option<f64>,
    pub custom_fee: Option<f64>,
}

// Модель для таблицы items
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Item {
    pub item_id: i32,
    pub order_id: i32,  // Внешний ключ на orders
    pub chrt_id: i32,
    pub track_number: String,
    pub price: Option<f64>,
    pub rid: String,
    pub name: String,
    pub sale: Option<f64>,
    pub size: String,
    pub total_price: Option<f64>,
    pub nm_id: i32,
    pub brand: String,
    pub status: i32,
}

// Модель для таблицы delivery
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Delivery {
    pub delivery_id: i32,
    pub order_id: i32,  // Внешний ключ на orders
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub city: String,
    pub address: String,
    pub region: String,
    pub email: String,
}
