use thiserror::Error;

#[derive(Error, Debug)]
pub enum SavepointError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("invalid savepoint value: {0}")]
    InvalidSavepoint(String),
}
