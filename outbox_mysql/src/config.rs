use serde::Deserialize;

fn default_table_name() -> String {
    String::from("outbox_events")
}

fn default_id_column_name() -> String {
    String::from("id")
}

fn default_topic_column_name() -> String {
    String::from("topic")
}

fn default_key_column_name() -> String {
    String::from("key")
}

fn default_payload_column_name() -> String {
    String::from("payload")
}

fn default_created_at_column_name() -> String {
    String::from("created_at")
}

#[derive(Deserialize, Debug, Clone)]
pub struct OutboxTableConfig {
    #[serde(default = "default_table_name")]
    pub table_name: String,

    #[serde(default = "default_id_column_name")]
    pub id_column_name: String,

    #[serde(default = "default_topic_column_name")]
    pub topic_column_name: String,

    #[serde(default = "default_key_column_name")]
    pub key_column_name: String,

    #[serde(default = "default_payload_column_name")]
    pub payload_column_name: String,

    #[serde(default = "default_created_at_column_name")]
    pub created_at_column_name: String,
}
