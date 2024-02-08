use std::{process::exit, sync::atomic::AtomicU32};
use std::sync::Mutex;
use logger::StructLogger;
use settings::{FileMethods, Settings};

use crate::{models::User, helpers::{Date, DateTimeFormat}, state_updater::{DateState}};

pub struct AppState
{
    copies_count: AtomicU32,
    errors_count: AtomicU32,
    errors: Mutex<Vec<String>>,
    settings: Mutex<Settings>,
    current_date: Mutex<Date>,
}
impl Default for AppState
{
    fn default() -> Self 
    {
        let settings = Settings::load();
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
            copies_count: AtomicU32::new(0),
            errors_count: AtomicU32::new(0),
            errors: Mutex::new(vec![]),
            settings: Mutex::new(settings.unwrap()),
            current_date: Mutex::new(*Date::now())
        }
    }
}
impl From<&AppState> for DateState
{
    fn from(value: &AppState) -> Self 
    {
        let d = value.current_date.lock().unwrap();
        Self
        {
            current_date: d.val.clone()
        }
    }
}

// impl From<&AppState> for DateState
// {
//     fn from(value: &AppState) -> Self 
//     {
//         Self
//         {
//             errors_count : value.errors_count.load(std::sync::atomic::Ordering::SeqCst),
//             copies_count: value.copies_count.load(std::sync::atomic::Ordering::SeqCst),
//             users_count: value.counts.users_count.load(std::sync::atomic::Ordering::SeqCst),
//             ordered_count: value.counts.ordered_count.load(std::sync::atomic::Ordering::SeqCst),
//             buisness_trip_count: value.counts.buisness_trip_count.load(std::sync::atomic::Ordering::SeqCst),
//         }
//     }
// }


// impl From<&AppState> for CountsState
// {
//     fn from(value: &AppState) -> Self 
//     {
//         let vac = value.users_with_statuses.lock().unwrap();
//         let dis = value.current_disease_users.lock().unwrap();
//         Self
//         {
//             diseases_count : value.diseases_count.load(std::sync::atomic::Ordering::SeqCst),
//             vacations_count: value.vacations_count.load(std::sync::atomic::Ordering::SeqCst),
//             users_count: value.users_count.load(std::sync::atomic::Ordering::SeqCst),
//             ordered_count: value.ordered_count.load(std::sync::atomic::Ordering::SeqCst),
//             buisness_trip_count: value.buisness_trip_count.load(std::sync::atomic::Ordering::SeqCst),
//             current_date: Date::now().val,
//             current_disease_users: dis.clone(),
//             users_with_statuses: vac.clone(),
//         }
//     }
// }

impl AppState
{
    pub fn set_errors_count(&self, count: u32)
    {
        let _update = self.errors_count.fetch_update(
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
            |_| Some(count));
    }
    fn set_copies_count(&self, count: u32)
    {
        let _update = self.copies_count.fetch_update(
            std::sync::atomic::Ordering::SeqCst,
            std::sync::atomic::Ordering::SeqCst,
            |_| Some(count));
    }
    pub fn get_settings(&self) -> Settings
    {
        let guard = self.settings.lock().unwrap();
        guard.clone()
    }
}
