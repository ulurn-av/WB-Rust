use crate::db::connection::Database;
use crate::order::models::{Order, Test};

pub struct OrderService {
    pub database: Database,
}

impl OrderService {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    // pub async fn create_order(&self, order: Order) -> Result<Order, String> {
    //
    // }

    pub async fn get_order_by_uid(&self, order_uid: &str) -> Result<Order, String> {
        match sqlx::query_as::<_, Order>(
            "SELECT * FROM public.order WHERE id = $1"
        )
            .bind(order_uid)
            .fetch_one(&self.database.pool)
            .await
        {
            Ok(order) => Ok(order),
            Err(err) => Err(format!("Failed to get order: {:?}", err)),
        }
    }
}