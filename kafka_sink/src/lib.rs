use futures::future::join_all;
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

pub struct KafkaRecord {
    pub topic: String,
    pub key: String,
    pub payload: String,
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

    pub async fn publish_events(&self, records: Vec<KafkaRecord>) -> Result<(), SynkError> {
        if records.len() == 0 {
            return Ok(());
        }

        self.producer.begin_transaction()?;

        let tasks = records.iter().map(|record| {
            self.producer.send(
                FutureRecord::to(record.topic.as_str())
                    .payload(record.payload.as_str())
                    .key(record.key.as_str()),
                Timeout::Never,
            )
        });

        join_all(tasks).await;

        self.producer
            .commit_transaction(Timeout::After(Duration::from_secs(5)))?;

        Ok(())
    }
}
