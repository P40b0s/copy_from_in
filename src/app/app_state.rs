use std::{sync::{Mutex, Arc}, path::Path, process::exit, rc::Rc};

use logger::error;
use once_cell::sync::OnceCell;
use crate::{settings::Settings, io};
use clap::Parser;
use super::Args;
pub static STATE: OnceCell<Mutex<AppState>> = OnceCell::new();
pub static LOG: OnceCell<Mutex<Vec<String>>> = OnceCell::new();
pub struct AppState
{
    pub settings : Settings,
    pub args: super::Args
}


impl AppState
{
    pub fn initialize()
    {
        LOG.set(Mutex::new(vec![]));
        logger::StructLogger::initialize_logger_with_callback(|log|
        {
            LOG.get().unwrap().lock().unwrap().push(log);
        });

        let settings = Settings::initialize();
        if settings.is_none()
        {
            exit(0x0100);
        }
        let args = Args::try_parse();
        if args.is_err()
        {
            error!("{}", args.unwrap_err());
            STATE.set(Mutex::new(AppState 
            {
                settings: settings.unwrap(),
                args: Args::default()
            }));
        }
        else 
        {
            STATE.set(Mutex::new(AppState 
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
   
    // pub fn get_settings() -> &'static Settings
    // {
    //     let s = STATE.get().unwrap();
    //     &s.lock().unwrap().settings
        
    // }
   
}

// pub struct Excludes
// {
//     pub dirs: Vec<String>
// }
// impl Excludes
// {
//     pub fn get() -> Vec<String>
//     {
//         STATE.get().unwrap().lock().unwrap().excludes.dirs
//     }
//     pub fn in_excludes(dir: &String) -> bool
//     {
//         if STATE.get().unwrap().lock().unwrap().excludes.dirs.contains(dir)
//         {
//             return true;
//         }
//         return false;
//     }
//     pub fn add(dir: &String)
//     {
//         STATE.get().unwrap().lock().unwrap().excludes.dirs.push(dir.to_owned());
//     }
//     pub fn serialize()
//     {
//         let file_name = STATE.get().unwrap().lock().unwrap().settings.except_dirs_file;
//         crate::io::serialize(STATE.get().unwrap().lock().unwrap().excludes.dirs, &file_name, None);
//     }
//     pub fn deserialize() -> Excludes
//     {
//         let file_name = STATE.get().unwrap().lock().unwrap().settings.except_dirs_file;
//         let file_name = Path::new(&file_name);
//         let ex = crate::io::deserialize::<Vec<String>>(&file_name);
//         Excludes {dirs: ex.1}
//     }
// }









