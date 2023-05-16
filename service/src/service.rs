use kafka_sink::{KafkaRecord, KafkaSinkService};
use outbox::OutboxService;
use savepoint::SavePointService;
use std::time::Duration;
use tokio::time::interval;

pub struct ChaparService {
    outbox_svc: OutboxService,
    kafka_sink_svc: KafkaSinkService,
    savepoint_svc: SavePointService,
    tick_interval: Duration,
    batch_size: u32,
}

impl ChaparService {
    pub fn new(
        outbox_svc: OutboxService,
        kafka_sink_svc: KafkaSinkService,
        savepoint_svc: SavePointService,
        tick_interval: Duration,
        batch_size: u32,
    ) -> Self {
        ChaparService {
            outbox_svc,
            kafka_sink_svc,
            savepoint_svc,
            tick_interval,
            batch_size,
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        let mut ticker = interval(self.tick_interval);

        loop {
            ticker.tick().await;
            self.process_new_events().await?;
        }

        // Ok(())
    }

    async fn process_new_events(&self) -> Result<usize, String> {
        let last_id = self.savepoint_svc.load().await.unwrap();

        let events = self
            .outbox_svc
            .get_events_from_id(last_id, Some(self.batch_size))
            .await
            .map_err(|e| e.to_string())?;

        if events.len() == 0 {
            info!("no new events detected, last id: {}", last_id);
            return Ok(events.len());
        }

        info!(
            "received new events, count: {}, last id: {}",
            events.len(),
            events.last().unwrap().id
        );

        let kafka_events = events
            .iter()
            .map(|event| KafkaRecord {
                topic: event.topic.clone(),
                key: event.key.clone(),
                payload: event.payload.clone(),
            })
            .collect();

        self.kafka_sink_svc
            .publish_events(kafka_events)
            .await
            .map_err(|_| "cannot publish kafka events")?;

        self.savepoint_svc
            .save(events.last().unwrap().id)
            .await
            .unwrap();

        info!(
            "published events into kafka, last id: {}",
            events.last().unwrap().id
        );

        Ok(events.len())
    }
}
