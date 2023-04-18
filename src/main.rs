mod copy;
mod settings;
mod app;
pub use app::{AppState};
pub use io::DirectoriesSpy;
use logger::debug;
mod io;

use std::{path::{PathBuf}, thread, time::Duration, process::{exit, ExitCode}};
extern crate chrono;
use chrono::Local;
use settings::{Settings, Task};


const DATE_FORMAT_STR: &'static str = "%Y-%m-%d][%H:%M:%S";
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main()
{
    //let config = app::app_config::AppConfig::load().unwrap_or_default();
    
    AppState::initialize();
    //TODO обработка аргументов до запуска основного функционала
    DirectoriesSpy::process_tasks();
    let delay = std::time::Duration::from_secs(3);

    loop
    {
        //println!("sleeping for 3  sec ");
        std::thread::sleep(delay);
    }
}

// fn run_process(settings: &Settings, except: &mut Vec<String>)
// {
//     if let Some(dirs) = copy::get_dirs(settings)
//     {
//         for d in dirs
//         {
//             if !except.contains(&d)
//             {
//                 except.push(d.clone());
//                 let mut target: PathBuf = PathBuf::from(settings.out_dir.as_path());
//                 let mut source: PathBuf = PathBuf::from(settings.in_dir.as_path());
//                 target.push(d.as_str());
//                 source.push(d.as_str());
//                 let dt = Local::now();
//                 println!("{} Обнаружена директория {}, копирую в {}", dt.format(DATE_FORMAT_STR),  source.display(), target.display());
//                 let _ = copy::copy_recursively(source.as_path(), target.as_path());
//             }
//         }
//     }
// }