use std::{sync::Mutex, path::Path};

use once_cell::sync::OnceCell;
use crate::settings::Settings;
pub static STATE: OnceCell<Mutex<AppState>> = OnceCell::new();

pub struct AppState
{
    pub settings : Settings,
    pub log: Vec<String>,
    pub excludes: Excludes,
    
}


impl AppState
{
    pub fn initialize(settings : Settings)
    {
        let ex = Excludes::deserialize();
        STATE.set(Mutex::new(AppState 
        {
            settings,
            log: vec![],
            excludes: ex
        }));
    }
    pub fn initialize_logging()
    {
        logger::StructLogger::initialize_logger_with_callback(|log|
        {
            STATE.get().unwrap().lock().unwrap().log.push(log);
        })
    }
    pub fn get_settings() -> Settings
    {
        STATE.get().unwrap().lock().unwrap().settings
    }
   
}

pub struct Excludes
{
    pub dirs: Vec<String>
}
impl Excludes
{
    pub fn get() -> Vec<String>
    {
        STATE.get().unwrap().lock().unwrap().excludes.dirs
    }
    pub fn in_excludes(dir: &String) -> bool
    {
        if STATE.get().unwrap().lock().unwrap().excludes.dirs.contains(dir)
        {
            return true;
        }
        return false;
    }
    pub fn add(dir: &String)
    {
        STATE.get().unwrap().lock().unwrap().excludes.dirs.push(dir.to_owned());
    }
    pub fn serialize()
    {
        let file_name = STATE.get().unwrap().lock().unwrap().settings.except_dirs_file;
        crate::io::serialize(STATE.get().unwrap().lock().unwrap().excludes.dirs, &file_name, None);
    }
    pub fn deserialize() -> Excludes
    {
        let file_name = STATE.get().unwrap().lock().unwrap().settings.except_dirs_file;
        let file_name = Path::new(&file_name);
        let ex = crate::io::deserialize::<Vec<String>>(&file_name);
        Excludes {dirs: ex.1}
    }
}









