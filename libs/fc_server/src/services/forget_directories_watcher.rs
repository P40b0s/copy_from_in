use std::sync::Arc;
use hashbrown::HashSet;
use crate::state::AppState;

/// если есть настройка для отслеживания забытых директорий то начнется обработка указанной директории
pub async fn start_forget_directories_handler(app_state: Arc<AppState>)
{
    let settings = app_state.settings.lock().await;
    let forget = settings.forget_directories_watcher.clone();
    drop(settings);
    let mut hm = HashSet::new();
    if let Some(forget) = forget
    {
        let forget = Arc::new(forget);
        tokio::spawn(async move
        {
            loop
            {
                let forget_settings = Arc::clone(&forget);
                let dirs = utilites::io::get_dirs_async(&forget_settings.dir_path).await;
                if dirs.is_err()
                {
                    logger::error!("Ошибка сканирования директорий по пути {} -> {}", forget_settings.dir_path.display(), dirs.err().unwrap());
                    continue;
                }
                let dirs = dirs.unwrap();
                if dirs.is_empty()
                {
                    hm.clear();
                }
                for d in dirs
                {
                    if hm.contains(&d)
                    {
                        let old_path = forget_settings.dir_path.join(&d);
                        let iter = if let Some(s) = old_path.to_str()
                        {
                            if let Some(splited) = s.split_once("_iter_")
                            {
                                splited.1.parse::<u32>().unwrap_or_default() + 1
                            }
                            else
                            {
                                0u32
                            }
                        }
                        else
                        {
                            0u32
                        };
                        let new_dir_name = [uuid::Uuid::now_v7().to_string(), "_iter_".to_owned(), iter.to_string()].concat();
                        let new_name_path = forget_settings.dir_path.join(&new_dir_name);
                        if let Ok(_) = std::fs::rename(&old_path, &new_name_path)
                        {
                            hm.remove(&d);
                            hm.insert(new_dir_name);
                        }
                        else 
                        {
                            logger::error!("Ошибка переименования директории {} в директорию {}", old_path.display(), new_name_path.display());
                        }
                    }
                    else 
                    {
                        hm.insert(d);
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(forget_settings.cycle_time as u64)).await;
            }
        });
    }
}



#[cfg(test)]
mod tests
{
    use std::{path::PathBuf};

    #[test]
    fn test_dir_name()
    {
        let old_path = PathBuf::new().join("some_dir_iter_231");
        let iter = if let Some(s) = old_path.to_str()
        {
            if let Some(splited) = s.split_once("_iter_")
            {
                assert_eq!(splited.1, "231");
                1
            }
            else 
            {
                0    
            }
        }
        else 
        {
            0
        };
        println!("{}", iter);
    }
}