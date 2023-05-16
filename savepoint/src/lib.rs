use std::sync::atomic::{AtomicU32, Ordering};
use tokio::fs;

mod errors;
use errors::SavepointError;

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

    pub async fn save(&self, id: u32) -> Result<(), SavepointError> {
        // update local state
        self.check_point.store(id, Ordering::Relaxed);

        // save into file
        fs::write(self.storage_path.as_str(), id.to_string()).await?;

        Ok(())
    }

    pub async fn load(&self) -> Result<u32, SavepointError> {
        // loading id from local state
        let check_point = self.check_point.load(Ordering::Relaxed);
        if check_point > 0 {
            return Ok(check_point);
        }

        // if check_point is 0, load it from file

        let result = fs::read_to_string(self.storage_path.as_str()).await;

        match result {
            Err(err) => match err.kind() {
                // if file does not exists
                // we will using 0 as starting point
                tokio::io::ErrorKind::NotFound => Ok(0),
                _ => Err(SavepointError::IoError(err)),
            },
            Ok(content) => {
                let id: u32 = content
                    .trim()
                    .parse()
                    .map_err(|_| SavepointError::InvalidSavepoint(content))?;

                self.check_point.store(id, Ordering::Relaxed);
                Ok(id)
            }
        }
    }
}
