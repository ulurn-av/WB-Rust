use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: u32,
    payment_dt: u64,
    bank: String,
    delivery_cost: u32,
    goods_total: u32,
    custom_fee: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    chrt_id: u32,
    track_number: String,
    price: u32,
    rid: String,
    name: String,
    sale: u32,
    size: String,
    total_price: u32,
    nm_id: u32,
    brand: String,
    status: u32,
}


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: u32,
    date_created: String,
    oof_shard: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Test {
    order_uid: String,
}