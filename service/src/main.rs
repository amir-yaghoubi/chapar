mod service;
use kafka_sink::KafkaSinkService;
use outbox::OutboxService;
use savepoint::SavePointService;
use service::ChaparService;

#[tokio::main]
async fn main() -> Result<(), String> {
    let outbox_svc = OutboxService::new("mysql://root:1234@localhost:3306/outbox", 5)
        .await
        .map_err(|e| e.to_string())?;
    let kafka_sink_svc =
        KafkaSinkService::new("localhost:9092").map_err(|_| "cannot connect to kafka sink")?;

    let savepoint_svc = SavePointService::new("db.json".into());

    let chapar_svc = ChaparService::new(outbox_svc, kafka_sink_svc, savepoint_svc);
    chapar_svc.run().await.unwrap();

    Ok(())
}
