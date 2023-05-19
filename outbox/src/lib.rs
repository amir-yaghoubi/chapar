use time::PrimitiveDateTime;

#[derive(Debug)]
pub struct OutboxEvent {
    pub id: u32,
    pub topic: String,
    pub key: String,
    pub payload: String,
    pub created_at: PrimitiveDateTime,
}
