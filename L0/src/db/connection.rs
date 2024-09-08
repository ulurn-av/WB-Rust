use sqlx::PgPool;
use std::env;
use sqlx::postgres::PgPoolOptions;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Database, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("🔥DATABASE_URL must be set");

        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

        Ok(Self {pool})
    }
}
