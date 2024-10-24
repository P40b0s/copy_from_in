use std::{path::Path, sync::Mutex};

use hashbrown::HashMap;
use logger::warn;
use serde::{Deserialize, Serialize};

use crate::{io, CopyModifier, FileMethods, Task, ValidationError, EXCLUDES};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings
{
    pub tasks: Vec<Task>
}
impl Default for Settings
{
    fn default() -> Self 
    {
        Settings 
        { 
            tasks: vec![],
        }
    }
    
}
impl FileMethods for Settings
{
    const FILE_PATH: &'static str = "settings";
    const PATH_IS_ABSOLUTE: bool = false;
    fn validate(&self) -> Result<(), Vec<ValidationError>>
    {
        let mut errors: Vec<ValidationError> = vec![];
        for task in &self.tasks
        {
            //Проверяем директории только если есть активный фильтр
            if task.is_active
            {
                if let Ok(e) = task.source_dir.try_exists()
                {
                    if !e
                    {
                        let err = ["Директория", &task.source_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
                        errors.push(ValidationError::new(Some("source_directory".to_owned()), err));
                    }
                }
                if let Ok(e) = task.target_dir.try_exists()
                {
                    if !e
                    {
                        let err = ["Директория ", &task.target_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
                        errors.push(ValidationError::new(Some("target_directory".to_owned()), err));
                    }
                }
                if task.report_dir.to_str().is_some_and(|r| r != "")
                {
                    if let Ok(e) = task.report_dir.try_exists()
                    {
                        if !e
                        {
                            let err = ["Директория ", &task.target_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
                            errors.push(ValidationError::new(Some("report_directory".to_owned()), err));
                        }
                    }
                }
                
                if task.copy_modifier != CopyModifier::CopyAll
                && task.filters.document_types.len() == 0
                && task.filters.document_uids.len() == 0
                {
                    let err = ["Для копирования выбран модификатор ", &task.copy_modifier.to_string(), " но не определены фильтры, для данного модификатора необходимо добавить хоть один фильтр"].concat();
                    errors.push(ValidationError::new(Some("filters".to_owned()), err));
                }
            }
        }
        if errors.len() > 0
        {
            Err(errors)
        }
        else 
        {
            Ok(())
        }
    }
    ///Десериализовать обьект из файла
    /// # Arguments
    ///
    /// * `serializer` - Из какого формата десериализовывать файл
    ///
    fn load(serializer: io::Serializer) -> Result<Self, Vec<ValidationError>> 
    {
        let fp = Self::get_filename_with_extension(&serializer);
        let des: (bool, Self) = crate::io::deserialize(&fp, Self::PATH_IS_ABSOLUTE, serializer.clone());
        if !des.0
        {
            let _ = crate::io::serialize(Settings::default(), &fp, Self::PATH_IS_ABSOLUTE, serializer);
            warn!("Файл настроек не найден, создан новый файл {}, для работы программы необходимо его настроить", &fp);
            Ok(Settings::default())
        }
        else 
        {
            des.1.validate()?;
            des.1.load_tasks_exludes();
            Ok(des.1)
        }
    }
}

impl Settings
{
    ///Добавить к задаче имя директории, чтобы больше ее не копировать
    /// если возвращает true то директория успешно добавлена в список, если false то такая директория там уже есть
    pub fn add_to_exclude(task_name: &str, dir: &String) -> bool
    {
        let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
        if !guard.contains_key(task_name)
        {
            guard.insert(task_name.to_owned(), vec![dir.to_owned()]);
            return true;
        }
        else 
        {
            if let Some(ex) = guard.get_mut(task_name)
            {
                let d = dir.to_owned();
                if !ex.contains(&d)
                {
                    ex.push(dir.to_owned());
                    return true;
                }
                else 
                {
                    return false;
                }
            }
        }
        return false;
    }

    pub fn clear_exclude(task_name: &str)
    {
        let excl = EXCLUDES.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = excl.lock().unwrap();
        if guard.contains_key(task_name)
        {
            guard.remove(task_name);
        }
    }
    fn delete_from_exclude(task_name: &str, dir: &String)
    {
        let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
        if let Some(v) = guard.get_mut(task_name)
        {
            v.retain(|r| r != dir);
        }
    }
    ///Сохранить исключения текущей задачи в файл
    pub fn save_exclude(task_name: &str)
    {
        let concat_path = [task_name, ".task"].concat();
        let file_name = Path::new(&concat_path);
        let guard = EXCLUDES.get().unwrap().lock().unwrap();
        if let Some(vec) = guard.get(task_name)
        {
            if let Err(e) = io::serialize(vec, file_name, true, io::Serializer::Json)
            {
                logger::error!("Ошибка сохранения исключений списка {} -> {}", &concat_path, e);
            }
        }  
    }
    ///Загрузить все списки исключений из внешних файлов
    pub fn load_tasks_exludes(&self)
    {
        for t in &self.tasks
        {
            Self::load_exclude(t);
        }
    }
    ///загрузить исключение из файла
    pub fn load_exclude(task: &Task)
    {
        let excl = EXCLUDES.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = excl.lock().unwrap();
        if !guard.contains_key(task.name.as_str())
        {
            let file = [&task.name, ".task"].concat();
            let path = Path::new(&file);
            let mut ex: (bool, Vec<String>) = io::deserialize(&path, true, io::Serializer::Json);
            ex.1.sort();
            guard.insert(task.name.clone(), ex.1);
        }
    }
    ///удалить исключение из файла *.task
    pub fn del_exclude(t: &Task, packet_name: &str)
    {
        let excl = EXCLUDES.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = excl.lock().unwrap();
        let excludes = guard.get_mut(t.get_task_name()).unwrap();
        excludes.retain(|r| r != packet_name);
        drop(guard);
        Self::save_exclude(t.get_task_name());
    }
    ///Обрезать файл с исключениями (*.task) удаляет из файла все директории которые отсутсвуют в текущий момент 
    /// по пути source_dir в текущей задаче
    pub fn truncate_excludes(&self) -> u32
    {
        let mut count: u32 = 0;
        for t in &self.tasks
        {
            count = 0;
            let mut guard = EXCLUDES.get().unwrap().lock().unwrap();
            let excludes = guard.get(t.get_task_name()).unwrap();
            let mut del: Vec<String> = vec![];
            if let Some(dirs) = io::get_dirs(t.get_source_dir()) 
            {
                for ex in excludes
                {
                    if dirs.contains(ex)
                    {
                        del.push(ex.to_owned());
                    }
                    else
                    {
                        count+=1;
                    }
                }
            }
            guard.insert(t.get_task_name().to_owned(), del);
            drop(guard);
            if count > 0
            {
                logger::info!("При проверке списка задачи {} исключено {} несуществующих директорий",  t.get_task_name(), count);
                Self::save_exclude(t.get_task_name());
            }
            else
            {
                logger::info!("При проверке списка задачи {} не найдено несуществующих директорий",  t.get_task_name());
            }
        }
        count
    }
}