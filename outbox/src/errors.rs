use thiserror::Error;

#[derive(Error, Debug)]
pub enum OutboxError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::error::Error),
}
