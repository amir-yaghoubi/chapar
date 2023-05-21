use dotenv::dotenv;
use kafka_sink::KafkaSinkService;
use simple_logger::SimpleLogger;
use std::time::Duration;
use tokio::signal;
use tokio::sync::mpsc;
#[macro_use]
extern crate log;
use config::ChaparConfig;
use outbox_mysql::OutboxService;
use savepoint::SavePointService;
use service::ChaparService;

mod config;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    SimpleLogger::new()
        .with_colors(true)
        .with_utc_timestamps()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let conf = envy::from_env::<ChaparConfig>().unwrap();

    let outbox_svc = OutboxService::new(
        conf.table_config,
        conf.mysql_address.as_str(),
        conf.mysql_max_connections,
    )
    .await
    .map_err(|e| e.to_string())
    .unwrap();

    info!("outbox mysql service initialized");

    let kafka_sink_svc = KafkaSinkService::new(conf.kafka_address.as_str())
        .map_err(|_| "cannot connect to kafka sink")
        .unwrap();

    info!("kafka sink service initialized");

    let savepoint_svc = SavePointService::new(conf.savepoint_path);

    let chapar_svc = ChaparService::new(
        outbox_svc,
        kafka_sink_svc,
        savepoint_svc,
        Duration::from_millis(conf.tick_interval),
        conf.batch_size,
    );

    let (sender, receiver) = mpsc::channel(1);

    let handler = tokio::spawn(async move {
        println!("hi");
        chapar_svc.run(receiver).await.unwrap();
        println!("bye");
    });

    signal::ctrl_c().await.unwrap();

    info!("shutting down server");

    sender.send(()).await.unwrap();
    handler.await.unwrap();

    info!("server shutted down gracefully");
}
