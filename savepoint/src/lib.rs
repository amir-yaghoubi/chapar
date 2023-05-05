pub struct SavePointService {
    storage_path: String,
}

impl SavePointService {
    pub fn new(storage_path: String) -> Self {
        SavePointService { storage_path }
    }

    pub async fn save(&self, id: u32) -> Result<(), String> {
        println!("saving id: {} into {}", id, self.storage_path);
        Ok(())
    }

    pub async fn load(&self) -> Result<u32, String> {
        Ok(5)
    }
}
