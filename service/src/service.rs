use kafka_sink::{KafkaRecord, KafkaSinkService};
use outbox::OutboxService;
use savepoint::SavePointService;
use std::time::{Duration, Instant};
use tokio::time::interval;

pub struct ChaparService {
    outbox_svc: OutboxService,
    kafka_sink_svc: KafkaSinkService,
    savepoint_svc: SavePointService,
}

impl ChaparService {
    pub fn new(
        outbox_svc: OutboxService,
        kafka_sink_svc: KafkaSinkService,
        savepoint_svc: SavePointService,
    ) -> Self {
        ChaparService {
            outbox_svc,
            kafka_sink_svc,
            savepoint_svc,
        }
    }

    pub async fn run(&self) -> Result<(), String> {
        let mut ticker = interval(Duration::from_millis(500));

        loop {
            ticker.tick().await;
            self.process_new_events().await?;
        }

        // Ok(())
    }

    async fn process_new_events(&self) -> Result<(), String> {
        let now = Instant::now();

        let last_id = self.savepoint_svc.load().await.unwrap();

        let events = self
            .outbox_svc
            .get_events_from_id(last_id, Some(5000))
            .await
            .map_err(|e| e.to_string())?;

        if events.len() == 0 {
            return Ok(());
        }

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

        println!("whole process: {}ms", now.elapsed().as_millis());

        Ok(())
    }
}
