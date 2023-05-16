use std::sync::atomic::{AtomicU32, Ordering};

use tokio::fs;
pub struct SavePointService {
    storage_path: String,
    check_point: AtomicU32,
}

impl SavePointService {
    pub fn new(storage_path: String) -> Self {
        SavePointService {
            storage_path,
            check_point: AtomicU32::new(0),
        }
    }

    pub async fn save(&self, id: u32) -> Result<(), String> {
        self.check_point.store(id, Ordering::Relaxed);

        fs::write(self.storage_path.as_str(), id.to_string())
            .await
            .unwrap();

        Ok(())
    }

    pub async fn load(&self) -> Result<u32, String> {
        let check_point = self.check_point.load(Ordering::Relaxed);
        if check_point > 0 {
            return Ok(check_point);
        }

        let result = fs::read_to_string(self.storage_path.as_str()).await;

        match result {
            Ok(content) => {
                let id = content.trim().parse().unwrap();
                self.check_point.store(id, Ordering::Relaxed);
                Ok(id)
            }
            Err(err) => {
                println!("error: {:?}", err);
                Ok(0)
                // let err = err.try_into().unwrap();
                // match err {
                //     tokio::io::ErrorKind::NotFound => Ok(0),
                //     _ => Err("error"),
                // }
            }
        }
    }
}
