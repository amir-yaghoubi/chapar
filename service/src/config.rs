use serde::Deserialize;

fn default_mysql_address() -> String {
    String::from("mysql://root:1234@localhost:3306/outbox")
}

fn default_mysql_max_connections() -> u32 {
    5
}

fn default_batch_size() -> u32 {
    5000
}

fn default_tick_interval() -> u64 {
    500
}

fn default_kafka_address() -> String {
    String::from("localhost:9092")
}

fn default_savepoint_path() -> String {
    String::from("chapar.db")
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChaparConfig {
    #[serde(default = "default_mysql_address")]
    pub mysql_address: String,

    #[serde(default = "default_mysql_max_connections")]
    pub mysql_max_connections: u32,

    #[serde(default = "default_batch_size")]
    pub batch_size: u32,

    #[serde(default = "default_tick_interval")]
    pub tick_interval: u64,

    #[serde(default = "default_kafka_address")]
    pub kafka_address: String,

    #[serde(default = "default_savepoint_path")]
    pub savepoint_path: String,

    #[serde(flatten)]
    pub table_config: outbox_mysql::OutboxTableConfig,
}
