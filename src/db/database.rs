use sqlx::{MySql, MySqlPool, Pool, Row, Transaction};
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pool: Arc<MySqlPool>,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = MySqlPool::connect(url).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}
