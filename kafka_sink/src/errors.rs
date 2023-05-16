use thiserror::Error;

#[derive(Error, Debug)]
pub enum SynkError {
    #[error("kafka error: {0}")]
    KafkaError(#[from] rdkafka::error::KafkaError),
}
