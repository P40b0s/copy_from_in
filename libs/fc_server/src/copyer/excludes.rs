use std::{ops::Deref, path::{Path, PathBuf}, sync::Mutex};

use hashbrown::HashMap;
use logger::error;
use once_cell::sync::{Lazy, OnceCell};
use settings::{Settings, Task};
use utilites::io::get_dirs;
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};

use crate::Error;

pub struct ExcludesService<T>(pub T) where T: ExcludesTrait;
impl<T> Deref for ExcludesService<T> where T: ExcludesTrait
{
    type Target = T;
    fn deref(&self) -> &Self::Target 
    {
        &self.0
    }
}

///Сервис для обработки пакетов-исключений с помощью redb storage
pub struct KeyValueStore
{
    db: Lazy<Mutex<Database>>
}
impl KeyValueStore
{
    pub fn new() -> Self
    {
        Self { db: Lazy::new(|| Mutex::new(Database::create("excludes.redb").unwrap())) }
    }
}

pub trait ExcludesTrait
{
    ///Добавить к задаче имя директории, чтобы больше ее не копировать
    ///если возвращает true то директория успешно добавлена в список, если false то такая директория там уже есть
    fn add(&self, task_name: &str, dir: &str) -> Result<bool, Error>;
    ///Удалить директорию из определенного таска, например если нужно заново пересканировать этот пакет
    fn delete(&self, task_name: &str, dir: &str) -> Result<(), Error>;
    ///Обрезать файл с исключениями (*.task) удаляет из файла все директории которые отсутсвуют в текущий момент 
    /// по пути source_dir в текущей задаче
    fn truncate(&self, tasks: &[Task]) -> Result<u64, Error>;
    ///Удаление таблицы
    fn clear(&self, task_name: &str) -> Result<(), Error>;
    ///Заменяет текущую таблицу если она есть, или создает новую
    fn replace(&self, task: &Task) -> Result<(), Error>;
}

impl ExcludesTrait for KeyValueStore
{
    fn add(&self, task_name: &str, dir: &str) -> Result<bool, Error>
    {
        let mut added = false;
        let db = self.db.lock().unwrap();
        let write_tr = db.begin_write()?;
        drop(db);
        {
            let td = TableDefinition::<&str, u8>::new(task_name);
            let mut table = write_tr.open_table(td)?;
            let get = table.get(dir)?;
            if get.is_none()
            {
                drop(get);
                let _ = table.insert(dir, 0)?;
                added = true;
            }
        }
        write_tr.commit()?;
        return Ok(added);
    }

    fn delete(&self, task_name: &str, dir: &str) -> Result<(), Error>
    {
        let db = self.db.lock().unwrap();
        let write_tr = db.begin_write()?;
        drop(db);
        {
            let td = TableDefinition::<&str, u8>::new(task_name);
            let mut table = write_tr.open_table(td)?;
            let _ = table.remove(dir)?;
        }
        write_tr.commit()?;
        Ok(())
    }

    fn truncate(&self, tasks: &[Task]) -> Result<u64, Error>
    {
        let mut count: u64 = 0;
        let mut table_count: u64 = 0;
        for t in tasks
        {
            if let Some(dirs) = get_dirs(t.get_source_dir())
            {
                let db = self.db.lock().unwrap();
                let write_tr = db.begin_write()?;
                drop(db);
                {
                    let td = TableDefinition::<&str, u8>::new(t.get_task_name());
                    table_count = write_tr.open_table(td.clone())?.len()?;
                    let _ = write_tr.delete_table(td.clone())?;
                    let mut table = write_tr.open_table(td)?;
                    for d in &dirs
                    {
                        let _ = table.insert(d.as_str(), 0)?;
                    }
                }
                write_tr.commit()?;
                //прибавляем к общему количеству разницу между количеством имеющихся директорий и количества записей в БД
                if table_count >= dirs.len() as u64
                {
                    count += table_count - dirs.len() as u64;
                }
            }
        }
        Ok(count)
    }

    fn clear(&self, task_name: &str) -> Result<(), Error>
    {
        let db = self.db.lock().unwrap();
        let write_tr = db.begin_write()?;
        drop(db);
        {
            let td = TableDefinition::<&str, u8>::new(task_name);
            let _ = write_tr.delete_table(td)?;
        }
        write_tr.commit()?;
        Ok(())
    }
    
    fn replace(&self, task: &Task) -> Result<(), Error> 
    {
        self.clear(task.get_task_name())?;
        if let Some(dirs) = get_dirs(&task.source_dir)
        {
            for d in &dirs
            {
                self.add(task.get_task_name(), d);
            }
        }
        Ok(())
    }
}


pub struct FileExcludes
{
    dictionary: OnceCell<Mutex<hashbrown::hash_map::HashMap<String, Vec<String>>>>
}
impl FileExcludes
{
    ///Загрузка всех исключений из файлов
    pub fn new(tasks: &[Task]) -> Self
    {
        let slf = Self { dictionary: OnceCell::new() };
        let dictionary = slf.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = dictionary.lock().unwrap();
        for t in tasks
        {
            if !guard.contains_key(t.name.as_str())
            {
                let file = [&t.name, ".task"].concat();
                let path = Path::new(&file);
                let ex: Result<Vec<String>, utilites::error::Error> = utilites::deserialize(&path, true, utilites::Serializer::Json);
                if let Ok(mut res) = ex
                {
                    res.sort();
                    guard.insert(t.name.clone(), res);
                };
            }
        }
        drop(guard);
        slf  
    }
    ///сохранение исключений в файл
    fn save(&self, task_name: &str)
    {
        let concat_path = [task_name, ".task"].concat();
        let file_name = Path::new(&concat_path);
        let guard = self.dictionary.get().unwrap().lock().unwrap();
        if let Some(vec) = guard.get(task_name)
        {
            if let Err(e) = utilites::serialize(vec, file_name, true, utilites::Serializer::Json)
            {
                logger::error!("Ошибка сохранения исключений списка {} -> {}", &concat_path, e);
            }
        }  
    }
}

impl ExcludesTrait for FileExcludes
{
    fn add(&self, task_name: &str, dir: &str) -> Result<bool, Error>
    {
        
        let value = self.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = value.lock().unwrap();
        if !guard.contains_key(task_name)
        {
            guard.insert(task_name.to_owned(), vec![dir.to_owned()]);
            self.save(task_name);
            return Ok(true);
        }
        else 
        {
            if let Some(ex) = guard.get_mut(task_name)
            {
                let d = dir.to_owned();
                if !ex.contains(&d)
                {
                    ex.push(dir.to_owned());
                    self.save(task_name);
                    return Ok(true);
                }
                else 
                {
                    return Ok(false);
                }
            }
        }
        return Ok(false);
    }

    fn delete(&self, task_name: &str, dir: &str) -> Result<(), Error>
    {
       
        let mut guard = self.dictionary.get().unwrap().lock().unwrap();
        if let Some(v) = guard.get_mut(task_name)
        {
            v.retain(|r| r != dir);
        }
        self.save(task_name);
        Ok(())
    }

    fn truncate(&self, tasks: &[Task]) -> Result<u64, Error>
    {
        let mut count: u32 = 0;
        let mut task_count: u32 = 0;
        for t in tasks
        {
            let mut guard = self.dictionary.get().unwrap().lock().unwrap();
            let excludes = guard.get_mut(t.get_task_name()).unwrap();
            task_count = excludes.len() as u32;
            if let Some(dirs) = get_dirs(t.get_source_dir()) 
            {
                if task_count >= dirs.len() as u32
                {
                    count += task_count - dirs.len() as u32;
                }
                *excludes = dirs;
                self.save(t.get_task_name());
            }
            drop(guard);
            if count > 0
            {
                logger::info!("При проверке списка задачи {} исключено {} несуществующих директорий",  t.get_task_name(), count);
            }
        }
        Ok(count as u64)
    }

    fn clear(&self, task_name: &str) -> Result<(), Error>
    {
        let excl = self.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
        let mut guard = excl.lock().unwrap();
        if guard.contains_key(task_name)
        {
            guard.remove(task_name);
        }
        self.save(task_name);
        Ok(())
    }

    fn replace(&self, task: &Task) -> Result<(), Error> 
    {
        self.clear(task.get_task_name())?;
        if let Some(dirs) = get_dirs(&task.source_dir)
        {
            let excl = self.dictionary.get_or_init(|| Mutex::new(HashMap::new()));
            let mut guard = excl.lock().unwrap();
            let excludes = guard.get_mut(task.get_task_name()).unwrap();
            *excludes = dirs;
            self.save(task.get_task_name());
        }
        Ok(())
    }
}

#[cfg(test)]
mod db_tests
{
    use std::ops::Deref;
    use logger::debug;
    use super::{ExcludesService, ExcludesTrait};
   ///добавление двух таблиц и их последующее их удаление
    #[test]
    fn test_redb_create_clear()
    {
        //let b : Box<dyn ExcludesTrait> = Box::new(super::KeyValueStore::new());
        logger::StructLogger::new_default();
        let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "1");
        let add_2 = t.add(tasks[1], "1");
        debug!("{:?} {:?}", &add_1, &add_2);
        assert!(add_1.is_ok() && add_2.is_ok());
        let d = t.clear(tasks[0]);
        assert!(d.is_ok());
        let d2 = t.clear(tasks[1]);
        assert!(d2.is_ok());
    }
/// добавление и удаление значения в таблицу
    #[test]
    fn test_redb_add_del()
    {
        let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "1");
        assert!(add_1.unwrap());
        let del_1 = t.delete(tasks[0], "1");
        assert!(del_1.is_ok()); 
    }
    ///попытка добавить дубликат значения в таблицу
    #[test]
    fn test_redb_skip_add_double()
    {
         let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "21");
        let add_2 = t.add(tasks[0], "21");
        assert!(add_1.unwrap(), "Ошибка добавления значения");
        assert!(!add_2.unwrap(), "Значение добавлено, хотя это дубликат и добавлено быть не должно!");
        let del_1 = t.delete(tasks[0], "21");
        assert!(del_1.is_ok(), "Удаление прошло не успешно"); 
    }
}

#[cfg(test)]
mod files_tests
{
    use std::ops::Deref;
    use logger::debug;
    use super::{ExcludesService, ExcludesTrait};
   ///добавление двух таблиц и их последующее их удаление
    #[test]
    fn test_redb_create_clear()
    {
        let b : Box<dyn ExcludesTrait> = Box::new(super::KeyValueStore::new());
        logger::StructLogger::new_default();
        let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "1");
        let add_2 = t.add(tasks[1], "1");
        debug!("{:?} {:?}", &add_1, &add_2);
        assert!(add_1.is_ok() && add_2.is_ok());
        let d = t.clear(tasks[0]);
        assert!(d.is_ok());
        let d2 = t.clear(tasks[1]);
        assert!(d2.is_ok());
    }
/// добавление и удаление значения в таблицу
    #[test]
    fn test_redb_add_del()
    {
        let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "1");
        assert!(add_1.unwrap());
        let del_1 = t.delete(tasks[0], "1");
        assert!(del_1.is_ok()); 
    }
    ///попытка добавить дубликат значения в таблицу
    #[test]
    fn test_redb_skip_add_double()
    {
         let t = super::ExcludesService(super::KeyValueStore::new());
        let tasks = vec!["task_1", "task_2"];
        let add_1 = t.add(tasks[0], "21");
        let add_2 = t.add(tasks[0], "21");
        assert!(add_1.unwrap(), "Ошибка добавления значения");
        assert!(!add_2.unwrap(), "Значение добавлено, хотя это дубликат и добавлено быть не должно!");
        let del_1 = t.delete(tasks[0], "21");
        assert!(del_1.is_ok(), "Удаление прошло не успешно"); 
    }
}