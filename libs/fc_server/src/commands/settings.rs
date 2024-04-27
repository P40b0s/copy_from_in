use std::path::Path;
use std::sync::Arc;
use logger::{debug, error};
use settings::{FileMethods, Task};
use transport::Packet;
use crate::copyer::ExcludesCreator;
use crate::state::AppState;
use crate::Error;

pub async fn get_log(state: Arc<AppState>) -> Result<Vec<Packet>, Error>
{
    let log:Vec<Packet> = crate::copyer::get_full_log().await.into();
    Ok(log)
}
pub async fn get(state: Arc<AppState>) -> Result<Vec<Task>, Error>
{
    let settings = state.get_settings().await;
    Ok(settings.tasks)
}

pub async fn update(payload: Task, state: Arc<AppState>) -> Result<(), Error>
{
    let mut sett = state.settings.lock().await;
    let backup = sett.clone();
    if let Some(t) = sett.tasks.iter_mut().find(|f| &f.name == &payload.name)
    {
        *t = Task {
            generate_exclude_file: false,
            ..payload.clone()
        };
    }
    else 
    {
        sett.tasks.push(payload.clone());    
    }
    let save_state = sett.save(settings::Serializer::Toml).map_err(|e| Error::SettingsValidation(e));
    if let Err(e) = save_state.as_ref()
    {
        error!("Ошибка сохранения настроек! {}", &e.to_string());
        *sett = backup;
        save_state?
    }
    if payload.generate_exclude_file
    {
        Task::create_stoplist_file(&payload).await;
    }
    Ok(())
}

pub async fn delete(payload: Task, state: Arc<AppState>) -> Result<(), Error>
{
    let mut sett = state.settings.lock().await;
    debug!("Запрос удаления задачи {:?}",  &payload);
    if let Some(i) = sett.tasks.iter().position(|p| p.get_task_name() == payload.get_task_name())
    {
        sett.tasks.remove(i);
    }
    let _save_state = sett.save(settings::Serializer::Toml).map_err(|e| Error::SettingsValidation(e));
    drop(sett);
    let concat_path = [payload.get_task_name(), ".task"].concat();
    let file_name = Path::new(&concat_path);
    let path = Path::new(&std::env::current_dir().unwrap()).join(file_name);
    let _ = std::fs::remove_file(path);
    Ok(())
}