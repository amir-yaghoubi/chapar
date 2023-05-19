use futures::future::join_all;
use outbox::OutboxEvent;
use rdkafka::{
    producer::{FutureProducer, FutureRecord, Producer},
    util::Timeout,
    ClientConfig,
};
use std::time::Duration;

mod errors;
pub use errors::SynkError;

pub struct KafkaSinkService {
    producer: FutureProducer,
}

impl KafkaSinkService {
    pub fn new(kafka_brokers: &str) -> Result<Self, SynkError> {
        let producer: &FutureProducer = &ClientConfig::new()
            .set("bootstrap.servers", kafka_brokers)
            .set("transactional.id", "outbox-events-cdc")
            .set("message.timeout.ms", "5000")
            .create()?;

        producer.init_transactions(Timeout::After(Duration::from_secs(1)))?;

        Ok(KafkaSinkService {
            producer: producer.clone(),
        })
    }

    pub async fn publish_events(&self, events: Vec<OutboxEvent>) -> Result<(), SynkError> {
        if events.len() == 0 {
            return Ok(());
        }

        self.producer.begin_transaction()?;
        let tasks = events.iter().map(|event| {
            self.producer.send(
                FutureRecord::to(event.topic.as_str())
                    .key(event.key.as_str())
                    .payload(event.payload.as_str())
                    .timestamp(event.created_at.assume_utc().unix_timestamp()),
                Timeout::Never,
            )
        });

        join_all(tasks).await;

        self.producer
            .commit_transaction(Timeout::After(Duration::from_secs(5)))?;

        Ok(())
    }
}
