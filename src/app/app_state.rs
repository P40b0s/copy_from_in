use std::{sync::{Mutex}, path::Path, process::exit, rc::Rc};

use logger::error;
use once_cell::sync::OnceCell;
use clap::Parser;
use crate::settings::Settings;
use super::Args;
pub static STATE: OnceCell<Mutex<AppState>> = OnceCell::new();
pub struct AppState
{
    pub settings : Settings,
    pub args: super::Args
}

impl AppState
{
    pub fn initialize()
    {
        logger::StructLogger::initialize_logger();
        let settings = Settings::initialize();
        if settings.is_none()
        {
            exit(0x0100);
        }
        let args = Args::try_parse();
        if args.is_err()
        {
            error!("Ошибка распознавания агрументов: {} программа будет запущена с настройками по умолчанию", args.unwrap_err());
            let _ = STATE.set(Mutex::new(AppState 
            {
                settings: settings.unwrap(),
                args: Args::default()
            }));
        }
        else
        {
            let _ = STATE.set(Mutex::new(AppState 
            {
                settings: settings.unwrap(),
                args: args.unwrap()
            }));
        }
       
    }
    pub fn get_settings(&self) -> &Settings
    {
        &self.settings
    }

}






