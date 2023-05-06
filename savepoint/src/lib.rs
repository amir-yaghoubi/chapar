use tokio::fs;
pub struct SavePointService {
    storage_path: String,
}

impl SavePointService {
    pub fn new(storage_path: String) -> Self {
        SavePointService { storage_path }
    }

    pub async fn save(&self, id: u32) -> Result<(), String> {
        println!("saving id: {} into {}", id, self.storage_path);
        fs::write(self.storage_path.as_str(), id.to_string())
            .await
            .unwrap();
        Ok(())
    }

    pub async fn load(&self) -> Result<u32, String> {
        let result = fs::read_to_string(self.storage_path.as_str()).await;

        match result {
            Ok(content) => Ok(content.trim().parse().unwrap()),
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
