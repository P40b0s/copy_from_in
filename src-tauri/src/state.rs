use std::process::exit;
use std::sync::Mutex;
use settings::{FileMethods, Settings};

pub struct AppState
{
    pub settings: Mutex<Settings>,
}
impl Default for AppState
{
    fn default() -> Self 
    {
        let settings = Settings::load(settings::Serializer::Toml);
        if settings.is_err()
        {
            for e in settings.err().unwrap()
            {
                logger::error!("{}", e.to_string());
            }
            logger::error!("Ошибка десериализации файла настроек, выход из программы...");
            exit(01);
        }
        Self
        {
            settings: Mutex::new(settings.unwrap()),
        }
    }
}


impl AppState
{
    pub fn get_settings(&self) -> Settings
    {
        let guard = self.settings.lock().unwrap();
        guard.clone()
    }
}
