use tokio::sync::Mutex;
use settings::{FileMethods, Settings};

pub struct AppState
{
    pub settings: Mutex<Settings>,
}

impl AppState
{
    pub async fn get_settings(&self) -> Settings
    {
        let guard = self.settings.lock().await;
        guard.clone()
    }
}
