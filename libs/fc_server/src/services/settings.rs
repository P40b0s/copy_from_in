use std::path::Path;
use std::sync::Arc;
use logger::{debug, error};
use settings::{FileMethods, Task};
use crate::db::PacketTable;
use crate::state::AppState;
use crate::Error;

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
        let service = state.get_copy_service();
        let _ = service.excludes_service.replace(&payload).await;
        //Task::create_stoplist_file(&payload).await;
    }
    Ok(())
}
///Удаляем файл настроек, удаляем задачу из настроек и заново сериализуем в файл,  
/// удаляем список директорий и все записи связанные с этим таском из БД
pub async fn delete(task_name: &str, state: Arc<AppState>) -> Result<(), Error>
{
    let mut sett = state.settings.lock().await;
    debug!("Запрос удаления задачи {:?}",  task_name);
    if let Some(i) = sett.tasks.iter().position(|p| p.get_task_name() == task_name)
    {
        sett.tasks.remove(i);
    }
    let _save_state = sett.save(settings::Serializer::Toml).map_err(|e| Error::SettingsValidation(e));
    drop(sett);
    let concat_path = [task_name, ".task"].concat();
    let file_name = Path::new(&concat_path);
    let path = Path::new(&std::env::current_dir().unwrap()).join(file_name);
    let _ = std::fs::remove_file(path);
    PacketTable::delete_by_task_name(task_name, state.get_db_pool()).await;
    Ok(())
}