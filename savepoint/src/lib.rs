use std::sync::{Arc, Mutex};

use tokio::fs;
pub struct SavePointService {
    storage_path: String,
    check_point: Arc<Mutex<u32>>,
}

impl SavePointService {
    pub fn new(storage_path: String) -> Self {
        SavePointService {
            storage_path,
            check_point: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn save(&self, id: u32) -> Result<(), String> {
        println!("saving id: {} into {}", id, self.storage_path);

        let mut check_point = self.check_point.lock().unwrap();
        *check_point = id;

        fs::write(self.storage_path.as_str(), id.to_string())
            .await
            .unwrap();
        Ok(())
    }

    pub async fn load(&self) -> Result<u32, String> {
        let check_point = self.check_point.lock().unwrap().clone();

        if check_point > 0 {
            return Ok(check_point);
        }

        let result = fs::read_to_string(self.storage_path.as_str()).await;

        match result {
            Ok(content) => {
                let id = content.trim().parse().unwrap();
                let mut check_point = self.check_point.lock().unwrap();
                *check_point = id;
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
