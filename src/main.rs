mod copy;
mod settings;
mod app;
pub use app::{AppState, Excludes};
pub use io::DirectoriesSpy;
mod io;

use std::{path::{PathBuf}, thread, time::Duration, process::{exit, ExitCode}};
extern crate chrono;
use chrono::Local;
use settings::Settings;


const DATE_FORMAT_STR: &'static str = "%Y-%m-%d][%H:%M:%S";
fn main()
{
    //let config = app::app_config::AppConfig::load().unwrap_or_default();
    let settings = Settings::initialize();
    if settings.is_none()
    {
        exit(0x0100);
    }
    AppState::initialize(settings.unwrap());
    AppState::initialize_logging();
   
}

fn run_process(settings: &Settings, except: &mut Vec<String>)
{
    if let Some(dirs) = copy::get_dirs(settings)
    {
        for d in dirs
        {
            if !except.contains(&d)
            {
                except.push(d.clone());
                let mut target: PathBuf = PathBuf::from(settings.out_dir.as_path());
                let mut source: PathBuf = PathBuf::from(settings.in_dir.as_path());
                target.push(d.as_str());
                source.push(d.as_str());
                let dt = Local::now();
                println!("{} Обнаружена директория {}, копирую в {}", dt.format(DATE_FORMAT_STR),  source.display(), target.display());
                let _ = copy::copy_recursively(source.as_path(), target.as_path());
            }
        }
    }
}