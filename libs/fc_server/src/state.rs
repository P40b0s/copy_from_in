use std::{process::exit, sync::Arc};
use db_service::SqlitePool;
use logger::debug;
use tokio::sync::Mutex;
use settings::{FileMethods, Settings};

use crate::{copyer::{ExcludesService, ExcludesTrait, KeyValueStore, CopyerService}, Error};

pub struct AppState
{
    pub settings: Mutex<Settings>,
    pub copyer_service: Arc<CopyerService>,
    db_pool: Arc<SqlitePool>
}
impl AppState
{
    pub async fn initialize() -> Result<Self, Error>
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
        let pool = Arc::new(db_service::new_connection("medo").await?);
        debug!("Инициализация базы данных");
        super::db::initialize_db(Arc::clone(&pool)).await;
        Ok(Self
        {
            settings: Mutex::new(settings.unwrap()),
            copyer_service: Arc::new(CopyerService 
            {
                excludes:   Box::new(KeyValueStore::new())
            }),
            db_pool: pool
        })
    }
    pub fn get_db_pool(&self) -> Arc<SqlitePool>
    {
        Arc::clone(&self.db_pool)
    }
    pub fn get_service(&self) -> Arc<CopyerService>
    {
        Arc::clone(&self.copyer_service)
    }
}
// impl Default for AppState
// {
//     fn default() -> Self 
//     {
//         let settings = Settings::load(settings::Serializer::Toml);
//         if settings.is_err()
//         {
//             for e in settings.err().unwrap()
//             {
//                 logger::error!("{}", e.to_string());
//             }
//             logger::error!("Ошибка десериализации файла настроек, выход из программы...");
//             exit(01);
//         }
//         Self
//         {
//             settings: Mutex::new(settings.unwrap()),
//             db_pool: db_service::ge
//         }
//     }
// }


impl AppState
{
    pub async fn get_settings(&self) -> Settings
    {
        let guard = self.settings.lock().await;
        guard.clone()
    }
}
