use outbox::OutboxEvent;
use sqlx::{
    mysql::{MySqlPool, MySqlPoolOptions},
    Row,
};

pub mod config;
pub mod errors;
pub use config::OutboxTableConfig;
pub use errors::OutboxError;

pub struct OutboxService {
    query: String,
    pool: MySqlPool,
}

impl OutboxService {
    pub async fn new(
        table_config: OutboxTableConfig,
        url: &str,
        max_connections: u32,
    ) -> Result<Self, OutboxError> {
        let query = format!(
            "SELECT 
                `{}` as `id`, `{}` as `topic`,
                `{}` as `key`, `{}` as `payload`,
                `{}` as `created_at`
            FROM `{}`
            WHERE `id` > ? AND `id` <= ?
            ORDER BY `id`;",
            table_config.id_column_name,
            table_config.topic_column_name,
            table_config.key_column_name,
            table_config.payload_column_name,
            table_config.created_at_column_name,
            table_config.table_name,
        );

        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(OutboxService { pool, query })
    }

    pub async fn get_events_from_id(
        &self,
        last_id: u32,
        limit: Option<u32>,
    ) -> Result<Vec<OutboxEvent>, OutboxError> {
        let limit = limit.unwrap_or(100);

        let mut conn = self.pool.acquire().await?;

        let events = sqlx::query(&self.query)
            .bind(last_id)
            .bind(last_id + limit)
            .map(|row| OutboxEvent {
                id: row.get("id"),
                topic: row.get("topic"),
                key: row.get("key"),
                payload: row.get("payload"),
                created_at: row.get("created_at"),
            })
            .fetch_all(conn.as_mut())
            .await?;

        Ok(events)
    }
}
