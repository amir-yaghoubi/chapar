use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use time::PrimitiveDateTime;

pub mod errors;
pub use errors::Error;

pub struct OutboxService {
    pool: MySqlPool,
}

#[derive(sqlx::FromRow, Debug)]
pub struct OutboxEvent {
    pub id: u32,
    pub topic: String,
    pub key: String,
    pub payload: String,
    pub created_at: PrimitiveDateTime,
}

impl OutboxService {
    pub async fn new(url: &str, max_connections: u32) -> Result<Self, Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await
            .map_err(|_| Error::ConnectionError)?;

        Ok(OutboxService { pool })
    }

    pub async fn get_events_from_id(
        &self,
        last_id: u32,
        limit: Option<u32>,
    ) -> Result<Vec<OutboxEvent>, Error> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|_| Error::ConnectionError)?;

        let events =
            sqlx::query_as::<_, OutboxEvent>("SELECT * FROM outbox_events WHERE id > ? LIMIT ?")
                .bind(last_id)
                .bind(limit.unwrap_or_else(|| 100))
                .fetch_all(conn.as_mut())
                .await
                .map_err(|_| Error::ConnectionError)?;

        Ok(events)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {}
// }