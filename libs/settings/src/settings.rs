use logger::warn;
use serde::{Deserialize, Serialize};

use crate::{CopyModifier, FileMethods, Task, ValidationError};
use utilites::{Serializer, serialize, deserialize, serialize_to_file};

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
                for td in &task.target_dirs
                {
                    if let Ok(e) = td.try_exists()
                    {
                        if !e
                        {
                            let err = ["Директория ", &td.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
                            errors.push(ValidationError::new(Some("target_directory".to_owned()), err));
                        }
                    }
                }
                if task.report_dir.to_str().is_some_and(|r| r != "")
                {
                    if let Ok(e) = task.report_dir.try_exists()
                    {
                        if !e
                        {
                            let err = ["Директория ", &task.report_dir.to_str().unwrap_or("***"), " в задаче ", &task.name, " не существует!"].concat();
                            errors.push(ValidationError::new(Some("report_directory".to_owned()), err));
                        }
                    }
                }
                if task.timer < 15000
                {
                    let err = ["Таймер для задачи ", &task.name, " установлен на ", &(task.timer/1000).to_string(), "c. таймер не может быть меньше 15с."].concat();
                    errors.push(ValidationError::new(Some("timer".to_owned()), err));
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
    fn load(serializer: Serializer) -> Result<Self, Vec<ValidationError>> 
    {
        let fp = Self::get_filename_with_extension(&serializer);
        let des: Result<Self, utilites::error::Error> = deserialize(&fp, Self::PATH_IS_ABSOLUTE, serializer.clone());
        if let Ok(des) = des
        {
            des.validate()?;
            Ok(des)
        }
        else 
        {
            let _ = serialize(Settings::default(), &fp, Self::PATH_IS_ABSOLUTE, serializer);
            warn!("Файл настроек не найден, создан новый файл {}, для работы программы необходимо его настроить", &fp);
            Ok(Settings::default())
        }
    }
}

impl Settings
{
    ///Список имен задач с флагом visible=true
    pub fn get_visible_tasks_names(&self) -> Vec<String>
    {
        self.tasks.iter().filter(|t| t.visible).map(|t| t.name.clone()).collect()
    }
    pub fn get_tasks(&self) -> &[Task]
    {
        &self.tasks
    }
}